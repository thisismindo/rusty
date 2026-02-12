use crate::database::DbPool;
use serde_json::json;
use warp::{Rejection, Reply};

pub async fn liveness_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json!({
        "status": "ok"
    })))
}

pub async fn readiness_handler(db_pool: DbPool) -> Result<impl Reply, Rejection> {
    let db_ok = sqlx::query("SELECT 1").fetch_one(&db_pool).await.is_ok();

    if db_ok {
        Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "status": "ok",
                "database": "connected"
            })),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&json!({
                "status": "error",
                "database": "disconnected"
            })),
            warp::http::StatusCode::SERVICE_UNAVAILABLE,
        ))
    }
}
