/// App global error values
pub enum AppError {
    DbError(sqlx::Error),
    NotFound,
    BadRequest(String),
    Internal(String),
}