//! Session issuance and validation for Loco JWT authentication.

use chrono::{Duration, Utc};
use loco_rs::{auth::jwt, prelude::*};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::models::{sessions, users};

/// Creates a signed token and a matching server-side session record.
pub async fn issue_token(ctx: &AppContext, user: &users::Model) -> Result<String> {
    let config = ctx.config.get_jwt_config()?;
    let seconds = i64::try_from(config.expiration)
        .map_err(|_| Error::string("JWT expiration is outside the supported range"))?;
    let expires_at = Utc::now()
        .checked_add_signed(Duration::seconds(seconds))
        .ok_or_else(|| Error::string("JWT expiration is outside the supported range"))?
        .fixed_offset();

    let session_id = Uuid::new_v4();
    let mut claims = Map::new();
    claims.insert("sid".to_owned(), Value::String(session_id.to_string()));

    let token = jwt::JWT::new(&config.secret)
        .generate_token(config.expiration, user.pid.to_string(), claims)
        .map_err(|_| Error::string("could not issue authentication token"))?;

    // The token is returned only after the database record is committed.
    sessions::create(&ctx.db, user.id, session_id, expires_at).await?;
    Ok(token)
}

/// Resolves a JWT to an active persisted session and user. All authenticated
/// Control Plane handlers must use this helper (or a future equivalent
/// extractor) rather than accepting a stateless JWT alone.
pub async fn require_active_session(
    ctx: &AppContext,
    claims: &auth::JWT,
) -> Result<(users::Model, Uuid)> {
    let session_id = claims
        .claims
        .claims
        .get("sid")
        .and_then(Value::as_str)
        .and_then(|value| Uuid::parse_str(value).ok())
        .ok_or_else(|| Error::Unauthorized("inactive session".to_owned()))?;

    let user = users::Model::find_by_pid(&ctx.db, &claims.claims.pid)
        .await
        .map_err(|_| Error::Unauthorized("inactive session".to_owned()))?;

    if !sessions::is_active(&ctx.db, user.id, session_id).await? {
        return Err(Error::Unauthorized("inactive session".to_owned()));
    }

    Ok((user, session_id))
}
