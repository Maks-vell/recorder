use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator_derive::Validate;

use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;


/// DTO для получения настроек хранения видео и скриншотов.
///
/// Все поля являются обязательными. .
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StorageSettingsDto {
    /// Максимальное количество дней хранения записей (1-30).
    pub max_storage_days: i32,

    /// Интервал между видеофрагментами в минутах (1-60).
    pub video_interval_minutes: i32,

    /// Интервал между скриншотами в минутах (1-60).
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


/// DTO для обновления настроек хранения видео и скриншотов.
///
/// Все поля являются опциональными.
/// Если ни одно поле не указано — настройки не изменяются.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateStorageSettingsDto {
    /// Максимальное количество дней хранения записей (1-30).
    #[validate(range(min = 1, max = 30))]
    #[schema(example = 7)]
    pub max_storage_days: Option<i32>,

    /// Интервал между видеофрагментами в минутах (1-60).
    #[validate(range(min = 1, max = 60))]
    #[schema(example = 10)]
    pub video_interval_minutes: Option<i32>,

    /// Интервал между скриншотами в минутах (1-60).
    #[validate(range(min = 1, max = 60))]
    #[schema(example = 10)]
    pub screenshots_interval_minutes: Option<i32>,
}
