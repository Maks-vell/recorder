use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_pool(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create Postgres pool");

    MIGRATOR.run(&pool).await.unwrap();

    pool
}