use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStorageSettingsDto {
    pub max_storage_days: Option<i32>,
    pub video_interval_minutes: Option<i32>,
    pub screenshots_interval_minutes: Option<i32>,
}