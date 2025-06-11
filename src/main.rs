use tokio;
use tracing_subscriber;

use recorder::{app, config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let app = app::build_app().await;

    let addr = format!("{}:{}", config.server_url, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Server listening on {}", &addr);
}