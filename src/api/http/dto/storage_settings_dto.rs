use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StorageSettingsDto {
    pub max_storage_days: i32,
    pub video_interval_minutes: i32,
    pub screenshots_interval_minutes: i32,
}
impl From<StorageSettingsEntity> for StorageSettingsDto {
    fn from(entity: StorageSettingsEntity) -> Self {
        StorageSettingsDto {
            max_storage_days: entity.max_storage_days,
            video_interval_minutes: entity.video_interval_minutes,
            screenshots_interval_minutes: entity.screenshots_interval_minutes,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateStorageSettingsDto {
    #[schema(example = 7)]
    pub max_storage_days: Option<i32>,
    #[schema(example = 10)]
    pub video_interval_minutes: Option<i32>,
    #[schema(example = 10)]
    pub screenshots_interval_minutes: Option<i32>,
}
