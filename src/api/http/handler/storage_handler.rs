use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde_json::json;
use tracing::error;
use validator::Validate;

use crate::api::http::dto::storage_settings_dto::{StorageSettingsDto, UpdateStorageSettingsDto};
use crate::app::app_state::AppState;
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
    patch,
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
) -> Result<Json<StorageSettingsDto>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(errors) = payload.validate() {
        let response_body = json!({
            "error": "Validation failed",
            "details": errors
        });

        return Err((StatusCode::BAD_REQUEST, Json(response_body)));
    }

    match state
        .services
        .storage_service
        .update_settings(payload)
        .await
    {
        Ok(updated) => Ok(Json(updated.into())),
        Err(e) => {
            error!("Update error: {}", e);
            let response_body = json!({
                "error": "Internal server error"
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response_body)))
        }
    }
}
