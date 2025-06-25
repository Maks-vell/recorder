use crate::infrastructure::recorderdb_pg::repository::record_repository::RecordRepository;

/// Сервис работы с записями видео.
///
/// Предоставляет методы для работы с записями видео.
pub struct  RecordService {
    repository: RecordRepository,
}

impl RecordService {
    pub fn new(repository: RecordRepository) -> Self {
        Self { repository }
    }
}