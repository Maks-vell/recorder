use sqlx::PgPool;

pub struct RecordRepository {
    pub pool: PgPool,
}

impl RecordRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

}