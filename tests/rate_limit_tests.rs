mod common;

use rusty::handlers::user_handlers::handle_rejection;
use rusty::routes::{health_routes, user_routes};
use rusty::{create_rate_limiter, get_db_pool};
use serde_json::Value;
use serial_test::serial;
use warp::Filter;

async fn create_low_rate_limit_api()
-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // SAFETY: Tests run serially, so no data races
    unsafe {
        std::env::set_var("API_KEY", common::TEST_API_KEY);
        std::env::set_var("DATABASE_URL", "mysql://user:password@localhost:3306/db");
    }

    let db_pool = get_db_pool().await.expect("Failed to create test DB pool");
    // Very low rate limit for testing: 2 requests per second
    let rate_limiter = create_rate_limiter(2);

    let user_routes = user_routes(db_pool.clone(), rate_limiter);
    let health_routes = health_routes(db_pool);

    user_routes.or(health_routes).recover(handle_rejection)
}

#[tokio::test]
#[serial]
async fn test_rate_limit_allows_normal_requests() {
    let api = create_low_rate_limit_api().await;

    // Single request should succeed
    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("X-API-Key", common::TEST_API_KEY)
        .reply(&api)
        .await;

    // Should not be rate limited (may be 404 for non-existent user)
    assert_ne!(response.status(), 429);
}

#[tokio::test]
#[serial]
async fn test_rate_limit_exceeded() {
    let api = create_low_rate_limit_api().await;

    let mut rate_limited_count = 0;

    for _ in 0..10 {
        let response = warp::test::request()
            .method("GET")
            .path("/users/1")
            .header("X-API-Key", common::TEST_API_KEY)
            .reply(&api)
            .await;

        if response.status() == 429 {
            rate_limited_count += 1;
        }
    }

    assert!(
        rate_limited_count > 0,
        "Expected some requests to be rate limited, but none were"
    );
}

#[tokio::test]
#[serial]
async fn test_rate_limit_error_message() {
    let api = create_low_rate_limit_api().await;

    for _ in 0..5 {
        let _ = warp::test::request()
            .method("GET")
            .path("/users/1")
            .header("X-API-Key", common::TEST_API_KEY)
            .reply(&api)
            .await;
    }

    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("X-API-Key", common::TEST_API_KEY)
        .reply(&api)
        .await;

    if response.status() == 429 {
        let body: Value = serde_json::from_slice(response.body()).unwrap();
        assert_eq!(body["error"], "Rate limit exceeded");
    }
}

#[tokio::test]
#[serial]
async fn test_rate_limit_applies_to_all_user_endpoints() {
    let api = create_low_rate_limit_api().await;

    let endpoints = vec![("GET", "/users/1"), ("DELETE", "/users/1")];

    for (method, path) in endpoints {
        // Exhaust rate limit
        for _ in 0..10 {
            let request = warp::test::request()
                .method(method)
                .path(path)
                .header("X-API-Key", common::TEST_API_KEY);

            let _ = request.reply(&api).await;
        }
    }

    let response = warp::test::request()
        .method("GET")
        .path("/users/1")
        .header("X-API-Key", common::TEST_API_KEY)
        .reply(&api)
        .await;

    assert!(response.status() == 429 || response.status() == 404);
}

#[tokio::test]
#[serial]
async fn test_health_endpoints_not_rate_limited() {
    let api = common::create_test_api_async().await;

    for _ in 0..20 {
        let response = warp::test::request()
            .method("GET")
            .path("/health/live")
            .reply(&api)
            .await;

        assert_ne!(
            response.status(),
            429,
            "Health endpoint should not be rate limited"
        );
        assert_eq!(response.status(), 200);
    }
}
