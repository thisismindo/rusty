use crate::database::DbPool;
use crate::helpers::custom_error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use serde_json::json;
use validator::Validate;
use warp::{Rejection, Reply};

pub async fn create_user_handler(
    user: CreateUserRequest,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    user.validate()
        .map_err(|e| warp::reject::custom(AppError::Validation(e.to_string())))?;

    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(&db_pool)
        .await
        .map_err(|e| warp::reject::custom(AppError::Database(e)))?;

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn get_user_handler(id: i32, db_pool: DbPool) -> Result<impl Reply, Rejection> {
    let user: Option<User> = sqlx::query_as("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&db_pool)
        .await
        .map_err(|e| warp::reject::custom(AppError::Database(e)))?;

    match user {
        Some(user) => Ok(warp::reply::json(&user)),
        None => Err(warp::reject::custom(AppError::NotFound)),
    }
}

pub async fn update_user_handler(
    id: i32,
    user: UpdateUserRequest,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
    user.validate()
        .map_err(|e| warp::reject::custom(AppError::Validation(e.to_string())))?;

    let result = sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
        .bind(&user.name)
        .bind(&user.email)
        .bind(id)
        .execute(&db_pool)
        .await
        .map_err(|e| warp::reject::custom(AppError::Database(e)))?;

    if result.rows_affected() == 0 {
        return Err(warp::reject::custom(AppError::NotFound));
    }

    Ok(warp::reply::json(&json!({"status": "success"})))
}

pub async fn delete_user_handler(id: i32, db_pool: DbPool) -> Result<impl Reply, Rejection> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&db_pool)
        .await
        .map_err(|e| warp::reject::custom(AppError::Database(e)))?;

    if result.rows_affected() == 0 {
        return Err(warp::reject::custom(AppError::NotFound));
    }

    Ok(warp::reply::json(&json!({"status": "deleted"})))
}

#[allow(clippy::unused_async)] // Required by warp's rejection handler signature
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(app_err) = err.find::<AppError>() {
        let (status, message) = match app_err {
            AppError::Validation(msg) => (warp::http::StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound => (warp::http::StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Unauthorized(msg) => (warp::http::StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Database(e) => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Internal(msg) => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::RateLimited => (
                warp::http::StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
        };

        let json = warp::reply::json(&json!({
            "error": message,
        }));
        Ok(warp::reply::with_status(json, status))
    } else if err.is_not_found() {
        let json = warp::reply::json(&json!({
            "error": "Not found",
        }));
        Ok(warp::reply::with_status(
            json,
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if err.find::<warp::reject::MissingHeader>().is_some() {
        let json = warp::reply::json(&json!({
            "error": "Missing required header: x-api-key",
        }));
        Ok(warp::reply::with_status(
            json,
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else {
        Err(err)
    }
}
