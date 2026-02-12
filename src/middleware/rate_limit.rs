use crate::helpers::custom_error::AppError;
use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter as GovRateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use warp::{Filter, Rejection};

pub type RateLimiter = Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

pub fn create_rate_limiter(requests_per_second: u32) -> RateLimiter {
    let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
    Arc::new(GovRateLimiter::direct(quota))
}

pub fn with_rate_limit(
    limiter: RateLimiter,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .map(move || limiter.clone())
        .and_then(check_rate_limit)
        .untuple_one()
}

async fn check_rate_limit(limiter: RateLimiter) -> Result<(), Rejection> {
    match limiter.check() {
        Ok(()) => Ok(()),
        Err(_) => Err(warp::reject::custom(AppError::RateLimited)),
    }
}
