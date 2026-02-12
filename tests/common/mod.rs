#![allow(dead_code)] // Not all test files use all functions

use rusty::handlers::user_handlers::handle_rejection;
use rusty::routes::{health_routes, user_routes};
use rusty::{DbPool, create_rate_limiter, get_db_pool};
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use warp::Filter;

pub const TEST_API_KEY: &str = "test-api-key";

pub static DB_POOL: LazyLock<Arc<Mutex<Option<DbPool>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub async fn get_test_db_pool() -> DbPool {
    let mut pool_guard = DB_POOL.lock().await;
    if pool_guard.is_none() {
        // SAFETY: Tests run serially, so no data races
        unsafe {
            std::env::set_var("DATABASE_URL", "mysql://user:password@localhost:3306/db");
            std::env::set_var("API_KEY", TEST_API_KEY);
        }
        let pool = get_db_pool().await.expect("Failed to create test DB pool");
        *pool_guard = Some(pool);
    }
    pool_guard.clone().unwrap()
}

pub fn create_test_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    // SAFETY: Tests run serially, so no data races
    unsafe {
        std::env::set_var("API_KEY", TEST_API_KEY);
    }

    let db_pool = tokio::runtime::Handle::current().block_on(async { get_test_db_pool().await });
    let rate_limiter = create_rate_limiter(100);

    let user_routes = user_routes(db_pool.clone(), rate_limiter);
    let health_routes = health_routes(db_pool);

    user_routes.or(health_routes).recover(handle_rejection)
}

pub async fn create_test_api_async()
-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // SAFETY: Tests run serially, so no data races
    unsafe {
        std::env::set_var("API_KEY", TEST_API_KEY);
    }

    let db_pool = get_test_db_pool().await;
    let rate_limiter = create_rate_limiter(100);

    let user_routes = user_routes(db_pool.clone(), rate_limiter);
    let health_routes = health_routes(db_pool);

    user_routes.or(health_routes).recover(handle_rejection)
}

pub async fn cleanup_test_user(db_pool: &DbPool, email: &str) {
    let _ = sqlx::query("DELETE FROM users WHERE email = ?")
        .bind(email)
        .execute(db_pool)
        .await;
}
