mod common;

use common::create_test_api_async;
use serde_json::Value;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_liveness_endpoint() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/health/live")
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
#[serial]
async fn test_readiness_endpoint() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/health/ready")
        .reply(&api)
        .await;

    let body: Value = serde_json::from_slice(response.body()).unwrap();

    // Readiness endpoint returns 200 if DB connected, 503 if not
    if response.status() == 200 {
        assert_eq!(body["status"], "ok");
        assert_eq!(body["database"], "connected");
    } else {
        assert_eq!(response.status(), 503);
        assert_eq!(body["status"], "error");
        assert_eq!(body["database"], "disconnected");
    }
}

#[tokio::test]
#[serial]
async fn test_health_endpoints_no_auth_required() {
    let api = create_test_api_async().await;

    let live_response = warp::test::request()
        .method("GET")
        .path("/health/live")
        .reply(&api)
        .await;

    assert_eq!(live_response.status(), 200);

    let ready_response = warp::test::request()
        .method("GET")
        .path("/health/ready")
        .reply(&api)
        .await;

    assert_eq!(ready_response.status(), 200);
}
