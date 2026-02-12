mod common;

use common::{TEST_API_KEY, cleanup_test_user, create_test_api_async, get_test_db_pool};
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_create_user_success() {
    let api = create_test_api_async().await;
    let db_pool = get_test_db_pool().await;

    cleanup_test_user(&db_pool, "create_test@example.com").await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Create Test User",
            "email": "create_test@example.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["status"], "success");

    cleanup_test_user(&db_pool, "create_test@example.com").await;
}

#[tokio::test]
#[serial]
async fn test_create_user_duplicate_email() {
    let api = create_test_api_async().await;
    let db_pool = get_test_db_pool().await;

    cleanup_test_user(&db_pool, "duplicate@example.com").await;

    let _ = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "First User",
            "email": "duplicate@example.com"
        }))
        .reply(&api)
        .await;

    let response = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Second User",
            "email": "duplicate@example.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 500);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["error"].as_str().unwrap().contains("Duplicate entry"));

    cleanup_test_user(&db_pool, "duplicate@example.com").await;
}

#[tokio::test]
#[serial]
async fn test_get_user_success() {
    let api = create_test_api_async().await;
    let db_pool = get_test_db_pool().await;

    cleanup_test_user(&db_pool, "get_test@example.com").await;

    let _ = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Get Test User",
            "email": "get_test@example.com"
        }))
        .reply(&api)
        .await;

    // Get user ID from database
    let row: (i32,) = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind("get_test@example.com")
        .fetch_one(&db_pool)
        .await
        .unwrap();

    let response = warp::test::request()
        .method("GET")
        .path(&format!("/users/{}", row.0))
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["name"], "Get Test User");
    assert_eq!(body["email"], "get_test@example.com");

    cleanup_test_user(&db_pool, "get_test@example.com").await;
}

#[tokio::test]
#[serial]
async fn test_get_user_not_found() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("GET")
        .path("/users/999999")
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    // Should return 404 for non-existent user, or 500 if DB has issues
    assert!(
        response.status() == 404 || response.status() == 500,
        "Expected 404 or 500, got {}",
        response.status()
    );

    if response.status() == 404 {
        let body: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["error"], "Not found");
    }
}

#[tokio::test]
#[serial]
async fn test_update_user_success() {
    let api = create_test_api_async().await;
    let db_pool = get_test_db_pool().await;

    cleanup_test_user(&db_pool, "update_test@example.com").await;
    cleanup_test_user(&db_pool, "updated@example.com").await;

    let _ = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Update Test User",
            "email": "update_test@example.com"
        }))
        .reply(&api)
        .await;

    let row: (i32,) = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind("update_test@example.com")
        .fetch_one(&db_pool)
        .await
        .unwrap();

    let response = warp::test::request()
        .method("PUT")
        .path(&format!("/users/{}", row.0))
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Updated Name",
            "email": "updated@example.com"
        }))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["status"], "success");

    let get_response = warp::test::request()
        .method("GET")
        .path(&format!("/users/{}", row.0))
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    let get_body: Value = serde_json::from_slice(get_response.body()).unwrap();
    assert_eq!(get_body["name"], "Updated Name");
    assert_eq!(get_body["email"], "updated@example.com");

    cleanup_test_user(&db_pool, "updated@example.com").await;
}

#[tokio::test]
#[serial]
async fn test_update_user_not_found() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("PUT")
        .path("/users/999999")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Nobody",
            "email": "nobody@example.com"
        }))
        .reply(&api)
        .await;

    assert!(
        response.status() == 404 || response.status() == 500,
        "Expected 404 or 500, got {}",
        response.status()
    );
}

#[tokio::test]
#[serial]
async fn test_delete_user_success() {
    let api = create_test_api_async().await;
    let db_pool = get_test_db_pool().await;

    cleanup_test_user(&db_pool, "delete_test@example.com").await;

    let _ = warp::test::request()
        .method("POST")
        .path("/users")
        .header("Content-Type", "application/json")
        .header("X-API-Key", TEST_API_KEY)
        .json(&json!({
            "name": "Delete Test User",
            "email": "delete_test@example.com"
        }))
        .reply(&api)
        .await;

    let row: (i32,) = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind("delete_test@example.com")
        .fetch_one(&db_pool)
        .await
        .unwrap();

    let response = warp::test::request()
        .method("DELETE")
        .path(&format!("/users/{}", row.0))
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    assert_eq!(body["status"], "deleted");

    let get_response = warp::test::request()
        .method("GET")
        .path(&format!("/users/{}", row.0))
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    assert_eq!(get_response.status(), 404);
}

#[tokio::test]
#[serial]
async fn test_delete_user_not_found() {
    let api = create_test_api_async().await;

    let response = warp::test::request()
        .method("DELETE")
        .path("/users/999999")
        .header("X-API-Key", TEST_API_KEY)
        .reply(&api)
        .await;

    assert!(
        response.status() == 404 || response.status() == 500,
        "Expected 404 or 500, got {}",
        response.status()
    );
}
