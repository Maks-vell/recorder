use std::sync::Arc;
use sqlx::PgPool;

use crate::app::app_config::AppConfig;
use crate::app::app_container::AppContainer;

/// Storage and inject all main parts o the app
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<AppConfig>,
    pub services: Arc<AppContainer>,
}

impl AppState {
    pub fn new(db: PgPool, config: Arc<AppConfig>) -> Self {
        let services = AppContainer::new(&db);

        Self {
            db,
            config,
            services,
        }
    }
}