use crate::auth::with_api_key;
use crate::database::DbPool;
use crate::handlers::user_handlers::{
    create_user_handler, delete_user_handler, get_user_handler, update_user_handler,
};
use crate::middleware::{RateLimiter, with_rate_limit};
use warp::Filter;

#[allow(clippy::needless_pass_by_value)] // DbPool is cloned into route filters
pub fn user_routes(
    db_pool: DbPool,
    rate_limiter: RateLimiter,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let create_user = warp::path("users")
        .and(warp::post())
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_api_key())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_user_handler);

    let get_user = warp::path!("users" / i32)
        .and(warp::get())
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_api_key())
        .and(with_db(db_pool.clone()))
        .and_then(get_user_handler);

    let update_user = warp::path!("users" / i32)
        .and(warp::put())
        .and(with_rate_limit(rate_limiter.clone()))
        .and(with_api_key())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(update_user_handler);

    let delete_user = warp::path!("users" / i32)
        .and(warp::delete())
        .and(with_rate_limit(rate_limiter))
        .and(with_api_key())
        .and(with_db(db_pool.clone()))
        .and_then(delete_user_handler);

    create_user.or(get_user).or(update_user).or(delete_user)
}

fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
