pub enum AppError {
    DbError(sqlx::Error),
    NotFound,
    BadRequest(String),
    Internal(String),
}