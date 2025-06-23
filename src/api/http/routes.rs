use axum::Router;
use axum::routing::{get, patch};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::http::handler::storage_handler;
use crate::api::http::dto::storage_settings_dto;
use crate::app::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        storage_handler::get_storage_settings,
        storage_handler::update_storage_settings,
    ),
    components(schemas(
        storage_settings_dto::StorageSettingsDto,
        storage_settings_dto::UpdateStorageSettingsDto
    )),
    tags(
        (name = "Storage", description = "Storage management endpoints")
    )
)]
pub struct ApiDoc;

pub async fn init_routes(state: AppState) -> Router {
    Router::new()
        .route("/api", get(|| async { "Ok" }))
        .route(
            "/api/storage/settings",
            get(storage_handler::get_storage_settings),
        )
        .route(
            "/api/storage/settings",
            patch(storage_handler::update_storage_settings),
        )
        .merge(SwaggerUi::new("/api/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
