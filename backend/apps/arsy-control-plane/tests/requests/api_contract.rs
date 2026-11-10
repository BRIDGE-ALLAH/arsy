use arsy_control_plane::app::App;
use loco_rs::testing::prelude::*;

#[tokio::test]
async fn unmatched_api_route_returns_safe_structured_not_found_with_request_id() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/v1/route-that-does-not-exist").await;
        assert_eq!(response.status_code(), 404);

        let body: serde_json::Value =
            serde_json::from_str(&response.text()).expect("error response is JSON");
        assert_eq!(body["error"]["code"], "not_found");
        assert_eq!(
            body["error"]["message"],
            "The requested resource was not found."
        );
        assert!(
            body["error"]["request_id"].as_str().is_some(),
            "correlation ID is returned for support diagnostics"
        );
        assert!(!body.to_string().contains("backtrace"));
        assert!(!body.to_string().contains("DATABASE_URL"));
    })
    .await;
}

#[tokio::test]
async fn unsupported_method_returns_normalized_client_error() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request
            .post("/api/v1/health/ready")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), 405);
        let body: serde_json::Value =
            serde_json::from_str(&response.text()).expect("error response is JSON");
        assert_eq!(body["error"]["code"], "invalid_request");
        assert!(body["error"]["request_id"].as_str().is_some());
    })
    .await;
}
