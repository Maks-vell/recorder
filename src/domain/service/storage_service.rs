use sqlx::{Error, PgPool};

use crate::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;
use crate::infrastructure::recorder_pg::repository::storage_repository::StorageRepository;

pub struct StorageService {
    repository: StorageRepository,
}

impl StorageService {
    pub fn new(pool: PgPool) -> Self {
        let repository = StorageRepository { pool };
        Self { repository }
    }

    pub async fn get_settings(&self) -> Result<StorageSettingsEntity, anyhow::Error> {
        self.repository.get_settings().await
    }

    pub async fn update_settings(&self, new: UpdateStorageSettingsDto) -> Result<(), Error> {
        self.repository.update_settings_partial(new).await
    }
}