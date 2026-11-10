//! Persisted server-side session support for the Loco JWT authentication flow.

use chrono::{Duration, Utc};
use loco_rs::{auth::jwt, prelude::*};
use serde_json::{Map, Value};
use uuid::Uuid;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::models::{_entities::users as user_entity, sessions, users};

/// Creates a signed token and matching session record. The token is returned
/// only after the session has been durably written to PostgreSQL.
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

    sessions::create(&ctx.db, user.id, session_id, expires_at).await?;
    Ok(token)
}

/// Resolve an authenticated JWT to a live database session and user. Every
/// protected handler must call this after Loco's JWT extractor validates the
/// signature and expiry; a stateless JWT alone is not sufficient authorization.
pub async fn require_active_session(
    ctx: &AppContext,
    claims: &auth::JWT,
) -> Result<(users::Model, Uuid)> {
    let Some(session_id) = claims
        .claims
        .claims
        .get("sid")
        .and_then(Value::as_str)
        .and_then(|value| Uuid::parse_str(value).ok())
    else {
        return unauthorized("missing session identifier");
    };

    let Some(pid) = Uuid::parse_str(&claims.claims.pid).ok() else {
        return unauthorized("invalid token subject");
    };
    let user = user_entity::Entity::find()
        .filter(user_entity::Column::Pid.eq(pid))
        .one(&ctx.db)
        .await?;
    let Some(user) = user else {
        return unauthorized("unknown token subject");
    };

    if !sessions::is_active(&ctx.db, user.id, session_id).await? {
        return unauthorized("inactive or revoked session");
    }

    Ok((user, session_id))
}
