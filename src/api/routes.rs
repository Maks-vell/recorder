use axum::Router;
use axum::routing::get;

use crate::app::AppState;

pub async fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/api", get(|| async { "Ok" }))
        .route("/api/storage/settings", get())
        .with_state(state)
}
