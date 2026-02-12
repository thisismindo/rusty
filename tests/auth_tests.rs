mod common;

use common::{TEST_API_KEY, create_test_api_async};
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_missing_api_key() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .reply(&api)
        .await;

    assert_eq!(response.status(), 401);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("x-api-key"));
}

#[tokio::test]
#[serial]
async fn test_invalid_api_key() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("X-API-Key", "wrong-api-key")
        .reply(&api)
        .await;

    assert_eq!(response.status(), 401);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["error"], "Invalid API key");
}

#[tokio::test]
#[serial]
async fn test_valid_api_key() {
    let api = create_test_api_async().await;

    // Should not return 401 with valid API key (may return 404 for non-existent user)
    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    // Should not be unauthorized
    assert_ne!(response.status(), 401);
}

#[tokio::test]
#[serial]
async fn test_api_key_required_for_post() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": "Test",
            "email": "test@test.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
#[serial]
async fn test_api_key_required_for_put() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("PUT")
        .path("/users/1")
        .header("Content-Type", "application/json")
        .json(&json!({
            "name": "Test",
            "email": "test@test.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
#[serial]
async fn test_api_key_required_for_delete() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("DELETE")
        .path("/users/1")
        .reply(&api)
        .await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
#[serial]
async fn test_api_key_case_insensitive_header() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("x-api-key", TEST_API_KEY)
        .reply(&api)
        .await;

    assert_ne!(response.status(), 401);
}
