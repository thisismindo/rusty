mod common;

use common::{TEST_API_KEY, create_test_api_async};
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_invalid_email_format() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Test User",
            "email": "not-an-email"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 400);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("email"));
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("Invalid email format")
    );
}

#[tokio::test]
#[serial]
async fn test_empty_name() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "",
            "email": "valid@email.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 400);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("name"));
    assert!(body["error"].as_str().unwrap().contains("1-100 characters"));
}

#[tokio::test]
#[serial]
async fn test_name_too_long() {
    let api = create_test_api_async().await;

    let long_name = "a".repeat(101);

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": long_name,
            "email": "valid@email.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 400);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("name"));
    assert!(body["error"].as_str().unwrap().contains("1-100 characters"));
}

#[tokio::test]
#[serial]
async fn test_valid_name_at_boundary() {
    let api = create_test_api_async().await;
    let db_pool = common::get_test_db_pool().await;

    let name_100 = "a".repeat(100);
    let email = "boundary_test@example.com";

    common::cleanup_test_user(&db_pool, email).await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": name_100,
            "email": email
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);

    common::cleanup_test_user(&db_pool, email).await;
}

#[tokio::test]
#[serial]
async fn test_valid_single_char_name() {
    let api = create_test_api_async().await;
    let db_pool = common::get_test_db_pool().await;

    let email = "single_char_test@example.com";

    common::cleanup_test_user(&db_pool, email).await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "A",
            "email": email
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);

    common::cleanup_test_user(&db_pool, email).await;
}

#[tokio::test]
#[serial]
async fn test_update_validation_invalid_email() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("PUT")
        .path("/users/1")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Valid Name",
            "email": "invalid-email"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 400);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("email"));
}

#[tokio::test]
#[serial]
async fn test_update_validation_empty_name() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("PUT")
        .path("/users/1")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "",
            "email": "valid@email.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 400);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("name"));
}

#[tokio::test]
#[serial]
async fn test_missing_required_fields() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Test User"
        }))
        .reply(&api)
        .await;

    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[tokio::test]
#[serial]
async fn test_various_valid_email_formats() {
    let api = create_test_api_async().await;
    let db_pool = common::get_test_db_pool().await;

    let valid_emails = vec![
        "simple@example.com",
        "very.common@example.com",
        "user+tag@example.com",
        "user123@example.co.uk",
    ];

    for email in valid_emails {
        common::cleanup_test_user(&db_pool, email).await;

        let response = warp::test::request()
            .method("POST")
            .path("/users")
            .header("Content-Type", "application/json")
            .header("X-API-Key", TEST_API_KEY)
            .json(&json!({
                "name": "Email Test User",
                "email": email
            }))
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200, "Email '{email}' should be valid");

        common::cleanup_test_user(&db_pool, email).await;
    }
}
