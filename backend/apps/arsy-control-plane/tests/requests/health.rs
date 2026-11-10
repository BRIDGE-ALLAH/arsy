use arsy_control_plane::app::App;
use loco_rs::testing::prelude::*;

#[tokio::test]
async fn default_liveness_endpoint_returns_success() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/_ping").await;
        assert_eq!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
async fn readiness_endpoint_returns_ready_when_database_is_available() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/health/ready").await;
        assert_eq!(response.status_code(), 200);
        response.assert_json(&serde_json::json!({ "status": "ready" }));
    })
    .await;
}
