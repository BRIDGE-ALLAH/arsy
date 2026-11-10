use arsy_control_plane::app::App;
use loco_rs::testing::prelude::*;

#[tokio::test]
async fn region_catalog_returns_empty_array_when_unseeded() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/v1/catalog/regions").await;
        assert_eq!(response.status_code(), 200);
        response.assert_json(&serde_json::json!([]));
    })
    .await;
}

#[tokio::test]
async fn compute_catalog_returns_empty_array_when_unseeded() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request.get("/api/v1/catalog/compute-offerings").await;
        assert_eq!(response.status_code(), 200);
        response.assert_json(&serde_json::json!([]));
    })
    .await;
}

#[tokio::test]
async fn compute_catalog_rejects_invalid_region_id() {
    request::<App, _, _>(|request, _ctx| async move {
        let response = request
            .get("/api/v1/catalog/compute-offerings?region_id=not-a-uuid")
            .await;
        assert_eq!(response.status_code(), 400);
    })
    .await;
}
