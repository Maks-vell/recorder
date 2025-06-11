use axum::Router;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use super::api::routes::init_routes;
use super::config::Config;
use super::infrastructure::db::connection::init_pool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

pub async fn build_app() -> Router {
    let config = Arc::new(Config::from_env());

    let db = init_pool(&config.database_url).await;

    let state = AppState {
        db,
        config: config.clone(),
    };

    init_routes(state).await
}
