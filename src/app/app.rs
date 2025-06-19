use axum::Router;
use std::sync::Arc;

use crate::api::http::routes::init_routes;
use super::config::Config;
use crate::infrastructure::recorder_pg::connection::init_pool;
use crate::app::state::AppState;

pub async fn build_app() -> Router {
    let config = Arc::new(Config::from_env());
    let db = init_pool(&config.database_url).await;

    let state = AppState::new(db, config.clone());

    init_routes(state).await
}