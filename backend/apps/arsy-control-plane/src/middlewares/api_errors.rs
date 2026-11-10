use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    middleware,
    response::Response,
    Router as AxumRouter,
};
use loco_rs::{
    app::AppContext,
    controller::middleware::MiddlewareLayer,
    Result,
};
use serde_json::Value;

use crate::dtos::common::{ApiError, ApiErrorEnvelope};

pub struct ApiErrorMiddleware;

impl MiddlewareLayer for ApiErrorMiddleware {
    fn name(&self) -> &'static str {
        "arsy_api_errors"
    }

    fn config(&self) -> serde_json::Result<Value> {
        Ok(serde_json::json!({ "enabled": true }))
    }

    fn apply(&self, app: AxumRouter<AppContext>) -> Result<AxumRouter<AppContext>> {
        Ok(app.layer(middleware::map_response(normalize_error_response)))
    }
}

async fn normalize_error_response(mut response: Response) -> Response {
    let status = response.status();
    if !status.is_client_error() && !status.is_server_error() {
        return response;
    }

    let request_id = response
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    let payload = ApiErrorEnvelope {
        error: ApiError::for_status(status.as_u16(), request_id),
    };

    let body = match serde_json::to_vec(&payload) {
        Ok(body) => body,
        Err(_) => {
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            b"{\"error\":{\"code\":\"internal_error\",\"message\":\"An internal error occurred.\",\"request_id\":null}}".to_vec()
        }
    };

    *response.body_mut() = Body::from(body);
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.remove(header::CONTENT_LENGTH);
    headers.remove(header::TRANSFER_ENCODING);
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middleware_is_toggleable_by_the_loco_stack_contract() {
        assert_eq!(ApiErrorMiddleware.name(), "arsy_api_errors");
        assert_eq!(
            ApiErrorMiddleware.config().expect("serializable config"),
            serde_json::json!({ "enabled": true })
        );
    }
}
