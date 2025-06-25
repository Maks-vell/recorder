use std::sync::Arc;
use sqlx::PgPool;

use crate::domain::service::device_service::DeviceService;
use crate::domain::service::record_service::RecordService;
use crate::domain::service::storage_service::StorageService;

#[derive(Clone)]
pub struct DomainContainer {
    pub storage_service: Arc<StorageService>,
    pub record_service: Arc<RecordService>,
    pub device_service: Arc<DeviceService>
}

impl DomainContainer {
    pub fn new(db: &PgPool) -> Arc<DomainContainer>{
        let storage_service = Arc::new(StorageService::new(db.clone()));
        let device_service = Arc::new(DeviceService::new(db.clone()));
        let record_service = Arc::new(RecordService::new(db.clone()));

        Arc::new(DomainContainer {
            storage_service,
            device_service,
            record_service
        })
    }
}