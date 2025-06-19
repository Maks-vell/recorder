use axum::Router;
use axum::routing::get;

use crate::api::http::handler::storage_handler;
use crate::app::state::AppState;

pub async fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/api", get(|| async { "Ok" }))
        .route("/api/storage/settings", get(storage_handler::get_storage_settings))
        .with_state(state)
}
