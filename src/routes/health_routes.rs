use crate::database::DbPool;
use crate::handlers::health_handlers::{liveness_handler, readiness_handler};
use warp::Filter;

pub fn health_routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let liveness = warp::path!("health" / "live")
        .and(warp::get())
        .and_then(liveness_handler);

    let readiness = warp::path!("health" / "ready")
        .and(warp::get())
        .and(with_db(db_pool))
        .and_then(readiness_handler);

    liveness.or(readiness)
}

fn with_db(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
