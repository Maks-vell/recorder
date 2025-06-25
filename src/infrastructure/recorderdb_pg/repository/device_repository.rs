use sqlx::PgPool;

pub struct DeviceRepository {
    pub pool: PgPool,
}

impl DeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

}