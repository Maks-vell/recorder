use axum::{
    extract::State,
    Json,
};
use axum::http::StatusCode;

use crate::api::http::dto::storage_settings_dto::{StorageSettingsDto, UpdateStorageSettingsDto};
use crate::app::state::AppState;

#[utoipa::path(
    get,
    path = "/api/storage/settings",
    responses(
        (status = 200, description = "Storage settings", body = StorageSettingsDto)
    )
)]
pub async fn get_storage_settings(
    State(state): State<AppState>,
) -> Result<Json<StorageSettingsDto>, StatusCode> {
    let result = state
        .services
        .storage_service
        .get_settings()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result.into()))
}

#[utoipa::path(
    post,
    path = "/api/storage/settings",
    request_body = UpdateStorageSettingsDto,
    responses(
        (status = 200, description = "Storage settings updated"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_storage_settings(
    State(state): State<AppState>,
    Json(payload): Json<UpdateStorageSettingsDto>,
) -> Result<StatusCode, StatusCode> {
    state
        .services
        .storage_service
        .update_settings(payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}