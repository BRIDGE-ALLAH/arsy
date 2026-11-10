use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::users,
        users::{LoginParams, RegisterParams},
    },
    views::auth::{CurrentResponse, LoginResponse},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ForgotParams {
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ResetParams {
    pub token: String,
    #[validate(length(min = 12, max = 128, message = "password must be between 12 and 128 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MagicLinkParams {
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ResendVerificationParams {
    #[validate(email(message = "invalid email"))]
    pub email: String,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    if params.validate().is_err() {
        return bad_request("invalid registration input");
    }

    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            // Do not log user-supplied email addresses or other personal data.
            tracing::info!(error = %err, "could not register user");
            return format::json(());
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    format::json(())
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(State(ctx): State<AppContext>, Path(token): Path<String>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_verification_token(&ctx.db, &token).await else {
        return unauthorized("invalid token");
    };

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    if params.validate().is_err() {
        return bad_request("invalid email");
    }
    let email = params.email.trim().to_ascii_lowercase();
    let Ok(user) = users::Model::find_by_email(&ctx.db, &email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    if params.validate().is_err() {
        return bad_request("invalid password reset input");
    }

    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    if params.validate().is_err() {
        return unauthorized("Invalid credentials!");
    }

    let email = params.email.trim().to_ascii_lowercase();
    let Ok(user) = users::Model::find_by_email(&ctx.db, &email).await else {
        tracing::debug!("login attempt with unknown account");
        return unauthorized("Invalid credentials!");
    };

    let valid = user.verify_password(&params.password);

    // Email verification is mandatory for password authentication. Keep the
    // response identical for invalid passwords and unverified accounts.
    if !valid || user.email_verified_at.is_none() {
        return unauthorized("Invalid credentials!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

/// Magic link authentication provides a secure and passwordless way to log in to the application.
///
/// # Flow
/// 1. **Request a Magic Link**:
///    A registered user sends a POST request to `/magic-link` with their email.
///    If the email exists, a short-lived, one-time-use token is generated and sent to the user's email.
///    For security and to avoid exposing whether an email exists, the response always returns 200, even if the email is invalid.
///
/// 2. **Click the Magic Link**:
///    The user clicks the link (/magic-link/{token}), which validates the token and its expiration.
///    If valid, the server generates a JWT and responds with a [`LoginResponse`].
///    If invalid or expired, an unauthorized response is returned.
///
/// This flow enhances security by avoiding traditional passwords and providing a seamless login experience.
async fn magic_link(
    State(ctx): State<AppContext>,
    Json(params): Json<MagicLinkParams>,
) -> Result<Response> {
    if params.validate().is_err() {
        return bad_request("invalid email");
    }

    let email = params.email.trim().to_ascii_lowercase();
    let Ok(user) = users::Model::find_by_email(&ctx.db, &email).await else {
        // Do not reveal whether an account exists.
        tracing::debug!("magic-link request for unknown account");
        return format::empty_json();
    };

    if user.email_verified_at.is_none() {
        // Only verified accounts can start a passwordless sign-in flow.
        return format::empty_json();
    }

    let user = user.into_active_model().create_magic_link(&ctx.db).await?;
    AuthMailer::send_magic_link(&ctx, &user).await?;

    format::empty_json()
}

/// Verifies a magic link token and authenticates the user.
async fn magic_link_verify(
    Path(token): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_magic_token(&ctx.db, &token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return unauthorized("unauthorized!");
    };

    let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn resend_verification_email(
    State(ctx): State<AppContext>,
    Json(params): Json<ResendVerificationParams>,
) -> Result<Response> {
    if params.validate().is_err() {
        return bad_request("invalid email");
    }

    let email = params.email.trim().to_ascii_lowercase();
    let Ok(user) = users::Model::find_by_email(&ctx.db, &email).await else {
        tracing::debug!("verification resend requested for unknown account");
        return format::json(());
    };

    if user.email_verified_at.is_some() {
        tracing::info!(
            pid = user.pid.to_string(),
            "User already verified, skipping resend"
        );
        return format::json(());
    }

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;
    tracing::info!(pid = user.pid.to_string(), "Verification email re-sent");

    format::json(())
}

fn routes_at(prefix: &'static str) -> Routes {
    Routes::new()
        .prefix(prefix)
        .add("/register", post(register))
        .add("/verify/{token}", get(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/current", get(current))
        .add("/magic-link", post(magic_link))
        .add("/magic-link/{token}", get(magic_link_verify))
        .add("/resend-verification-mail", post(resend_verification_email))
}

/// Versioned authentication routes. New clients should use this prefix.
pub fn routes() -> Routes {
    routes_at("/api/v1/auth")
}

/// Temporary compatibility alias for clients using the original generated path.
/// Remove only in a separately announced breaking API release.
pub fn legacy_routes() -> Routes {
    routes_at("/api/auth")
}
