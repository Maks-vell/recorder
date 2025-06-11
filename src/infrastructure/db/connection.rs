use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create Postgres pool")
}