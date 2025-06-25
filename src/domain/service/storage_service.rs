use sqlx::{Error};

use crate::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;
use crate::infrastructure::recorderdb_pg::repository::storage_repository::StorageRepository;

/// Сервис работы с хранением.
///
/// Предоставляет методы для работы с хранением.
pub struct StorageService {
    repository: StorageRepository,
}

impl StorageService {
    /// Создаёт новый экземпляр `StorageService`.
    ///
    /// # Аргументы
    /// * `pool` - пул соединений с PostgreSQL.
    pub fn new(repository: StorageRepository) -> Self {
        Self { repository }
    }

    /// Получает текущие настройки хранения из базы данных.
    ///
    /// # Пример
    /// ```ignore
    /// let service = StorageService::new(pool);
    /// let settings = service.get_settings().await?;
    /// println!("{:?}", settings.max_storage_days);
    /// ```
    ///
    /// # Ошибки
    /// Возвращает ошибку, если значение не найдено.
    pub async fn get_settings(&self) -> Result<StorageSettingsEntity, anyhow::Error> {
        self.repository.get_settings().await
    }


    /// Частично обновляет настройки хранения.
    ///
    /// Обновляет только те поля, которые указаны в DTO.
    /// Остальные поля остаются неизменными.
    ///
    /// # Пример
    /// ```ignore
    /// use recorder::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
    ///
    /// let dto = UpdateStorageSettingsDto {
    ///     max_storage_days: Some(21),
    ///     ..Default::default()
    /// };
    /// let updated = service.update_settings(dto).await?;
    /// ```
    ///
    /// # Возвращает
    /// Обновлённую сущность `StorageSettingsEntity`.
    ///
    /// # Ошибки
    /// Возвращает ошибку, если запрос в базу данных не удался.
    pub async fn update_settings(&self, new: UpdateStorageSettingsDto) -> Result<StorageSettingsEntity, Error> {
        self.repository.update_settings_partial(new).await
    }
}