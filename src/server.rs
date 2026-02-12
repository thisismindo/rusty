use rusty::database::get_db_pool;
use rusty::handlers::user_handlers::handle_rejection;
use rusty::middleware::create_rate_limiter;
use rusty::routes::{health_routes, user_routes};
use tokio::signal;
use warp::Filter;

pub async fn run_server() {
    let db_pool = get_db_pool().await.expect("Failed to connect to database");

    // Create rate limiter: 100 requests per second
    let rate_limiter = create_rate_limiter(100);

    // Combine all routes
    let user_routes = user_routes(db_pool.clone(), rate_limiter);
    let health_routes = health_routes(db_pool);

    let routes = user_routes.or(health_routes).recover(handle_rejection);

    // Create server with graceful shutdown
    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 3030), shutdown_signal());

    println!("Server running on http://{addr}");

    server.await;

    println!("Server shut down gracefully");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => println!("\nReceived Ctrl+C, shutting down..."),
        () = terminate => println!("\nReceived SIGTERM, shutting down..."),
    }
}
