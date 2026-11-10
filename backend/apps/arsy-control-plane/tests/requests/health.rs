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

        let openapi = request.get("/api/v1/openapi.json").await;
        assert_eq!(openapi.status_code(), 200, "versioned OpenAPI document should be available");
        let openapi_body = openapi.text();
        assert!(openapi_body.contains("\"openapi\":\"3.1.0\""));
        assert!(openapi_body.contains("/api/v1/auth/login"));

        let versioned_auth = request.get("/api/v1/auth/current").await;
        assert_eq!(
            versioned_auth.status_code(),
            401,
            "versioned current-user route should require authentication"
        );

        // Existing generated clients continue to receive the original auth route
        // during the compatibility window.
        let legacy_auth = request.get("/api/auth/current").await;
        assert_eq!(legacy_auth.status_code(), 401);

        let readiness = request.get("/_readiness").await;
        assert_eq!(
            readiness.status_code(),
            200,
            "readiness should be healthy while the test database is available"
        );
    })
    .await;
}
