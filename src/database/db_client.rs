use crate::helpers::custom_error::AppError;
use sqlx::mysql::MySqlPool;

pub type DbPool = MySqlPool;

pub async fn get_db_pool() -> Result<DbPool, AppError> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Internal("DATABASE_URL environment variable not set".into()))?;

    MySqlPool::connect(&database_url)
        .await
        .map_err(AppError::from)
}
