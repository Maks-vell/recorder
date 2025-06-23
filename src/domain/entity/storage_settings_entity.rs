use serde::Serialize;
use sqlx::FromRow;

/// Настройки хранения видео в БД.
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct StorageSettingsEntity {
    /// Максимальное количество дней хранения записей (1-30).
    pub max_storage_days: i32,

    /// Интервал между видеофрагментами в минутах (1-60).
    pub video_interval_minutes: i32,

    /// Интервал между скриншотами в минутах (1-60).
    pub screenshots_interval_minutes: i32,
}