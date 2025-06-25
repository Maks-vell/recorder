use crate::infrastructure::recorderdb_pg::repository::device_repository::DeviceRepository;

/// Сервис работы с девайсами.
///
/// Предоставляет методы для работы с девайсами.
pub struct  DeviceService {
    repository: DeviceRepository,
}

impl DeviceService {
    pub fn new(repository: DeviceRepository) -> Self {
        Self { repository }
    }
}