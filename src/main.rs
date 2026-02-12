mod server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    server::run_server().await;
}
