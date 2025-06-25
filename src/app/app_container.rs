use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::service::device_service::DeviceService;
use crate::domain::service::record_service::RecordService;
use crate::domain::service::storage_service::StorageService;
use crate::infrastructure::recorderdb_pg::repository::device_repository::DeviceRepository;
use crate::infrastructure::recorderdb_pg::repository::record_repository::RecordRepository;
use crate::infrastructure::recorderdb_pg::repository::storage_repository::StorageRepository;

#[derive(Clone)]
pub struct AppContainer {
    pub storage_service: Arc<StorageService>,
    pub record_service: Arc<RecordService>,
    pub device_service: Arc<DeviceService>,
}

impl AppContainer {
    pub fn new(pool: &PgPool) -> Arc<AppContainer> {
        let storage_service = Arc::new(StorageService::new(StorageRepository::new(pool.clone())));
        let device_service = Arc::new(DeviceService::new(DeviceRepository::new(pool.clone())));
        let record_service = Arc::new(RecordService::new(RecordRepository::new(pool.clone())));

        Arc::new(AppContainer {
            storage_service,
            device_service,
            record_service,
        })
    }
}
