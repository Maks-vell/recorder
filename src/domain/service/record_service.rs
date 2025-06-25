use sqlx::PgPool;
use crate::infrastructure::recorderdb_pg::repository::record_repository::RecordRepository;

/// Сервис работы с записями видео.
///
/// Предоставляет методы для работы с записями видео.
pub struct  RecordService {
    repository: RecordRepository,
}

impl RecordService {
    pub fn new(pool: PgPool) -> Self {
        let repository = RecordRepository { pool };
        Self { repository }
    }
}