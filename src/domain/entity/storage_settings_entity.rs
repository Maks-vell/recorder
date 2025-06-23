use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct StorageSettingsEntity {
    pub max_storage_days: i32,
    pub video_interval_minutes: i32,
    pub screenshots_interval_minutes: i32,
}