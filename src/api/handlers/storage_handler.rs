use axum::{extract::State, http::Response};
use hyper::Body;
use crate::app::AppState;
use serde::Serialize;
use simd_json::to_vec;

#[derive(Serialize)]
pub struct StorageSettingsResponse {
    pub path_template: String,
    pub max_days: u32,
    pub min_free_space_gb: u64,
}

pub async fn get_storage_settings(
    State(state): State<AppState>,
) -> Response<Body> {
    let payload = StorageSettingsResponse {
        path_template: "/videos/{camera_id}/{date}/".to_string(),
        max_days: 30,
        min_free_space_gb: 10,
    };

    let json = to_vec(&payload).expect("Failed to serialize");

    Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap()
}