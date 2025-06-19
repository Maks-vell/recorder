use axum::{
    extract::State,
    Json,
};
use axum::http::StatusCode;
use crate::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
use crate::app::state::AppState;
use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;

pub async fn get_storage_settings(
    State(state): State<AppState>,
) -> Result<Json<StorageSettingsEntity>, StatusCode> {
    let result = state
        .services
        .storage_service
        .get_settings()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;


    Ok(Json(result))
}

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