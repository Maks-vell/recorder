use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use super::app_config::AppConfig;
use crate::api::http::routes::init_routes;
use crate::app::app_state::AppState;
use crate::infrastructure::recorderdb_pg::connection::init_pool;

pub struct App {}
impl App {
    pub async fn run() {
        tracing_subscriber::fmt()
            .pretty()
            .with_target(false)
            .with_level(true)
            .with_thread_ids(true)
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let app_config = Arc::new(AppConfig::from_env());
        let addr = format!("{}:{}", app_config.server_url, app_config.server_port);

        let db = init_pool(&app_config.database_url).await;

        let app_state = AppState::new(db, app_config);

        let routes = init_routes(app_state).await;

        let listener = TcpListener::bind(&addr).await.unwrap();

        eprintln!(
            "Server listening on {}:{}",
            listener.local_addr().unwrap().ip(),
            listener.local_addr().unwrap().port()
        );

        axum::serve(listener, routes).await.unwrap();
    }
}
