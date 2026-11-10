use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use loco_rs::prelude::*;
use sea_orm::ConnectionTrait;
use serde_json::json;

fn readiness_response(ready: bool) -> Response {
    if ready {
        (StatusCode::OK, Json(json!({ "status": "ready" }))).into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "not_ready" })),
        )
            .into_response()
    }
}

/// Returns readiness only when the database required by the Control Plane responds.
#[debug_handler]
async fn ready(State(ctx): State<AppContext>) -> Result<Response> {
    let database_ready = ctx.db.execute_unprepared("SELECT 1").await.is_ok();
    Ok(readiness_response(database_ready))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/api/v1/health").add("/ready", get(ready))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_dependency_maps_to_service_unavailable_without_details() {
        let response = readiness_response(false);
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn available_dependency_maps_to_ok() {
        let response = readiness_response(true);
        assert_eq!(response.status(), StatusCode::OK);
    }
}
