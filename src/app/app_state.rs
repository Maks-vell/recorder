use std::sync::Arc;
use sqlx::PgPool;

use crate::app::app_config::AppConfig;
use crate::domain::service::storage_service::StorageService;

/// Includes self all domain services
#[derive(Clone)]
pub struct ServiceContainer {
    pub storage_service: Arc<StorageService>
}

/// Storage and inject all main parts o the app
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<AppConfig>,
    pub services: Arc<ServiceContainer>,
}

impl AppState {
    pub fn new(db: PgPool, config: Arc<AppConfig>) -> Self {
        let storage_service = Arc::new(StorageService::new(db.clone()));

        let services = Arc::new(ServiceContainer {
            storage_service,
        });

        Self {
            db,
            config,
            services,
        }
    }
}