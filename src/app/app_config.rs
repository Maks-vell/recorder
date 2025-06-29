use std::env;

/// Configurations from .env
#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
    pub server_url: String,
}

impl AppConfig {
    /// Load configurations from .env
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            server_port: env::var("SERVER_PORT")
                .expect("PORT not set")
                .parse()
                .expect("PORT not number"),
            server_url: env::var("SERVER_URL").expect("SERVER_URL not set"),
        }
    }
}
