use arsy_control_plane::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn monitoring_endpoints_are_mounted() {
    request::<App, _, _>(|request, _ctx| async move {
        let ping = request.get("/_ping").await;
        assert_eq!(ping.status_code(), 200, "liveness should be healthy");
        assert!(
            ping.text().contains("\"ok\":true"),
            "liveness response should be a small JSON success value"
        );

        let health = request.get("/_health").await;
        assert_eq!(
            health.status_code(),
            200,
            "health endpoint should be healthy"
        );

        let readiness = request.get("/_readiness").await;
        assert_eq!(
            readiness.status_code(),
            200,
            "readiness should be healthy while the test database is available"
        );
    })
    .await;
}
