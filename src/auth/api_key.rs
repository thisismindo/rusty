use crate::helpers::custom_error::AppError;
use warp::{Filter, Rejection};

const API_KEY_HEADER: &str = "x-api-key";

pub fn with_api_key() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::<String>(API_KEY_HEADER)
        .and_then(validate_api_key)
        .untuple_one()
}

async fn validate_api_key(key: String) -> Result<(), Rejection> {
    let valid_key = std::env::var("API_KEY")
        .map_err(|_| warp::reject::custom(AppError::Internal("API_KEY not configured".into())))?;

    if key == valid_key {
        Ok(())
    } else {
        Err(warp::reject::custom(AppError::Unauthorized(
            "Invalid API key".into(),
        )))
    }
}
