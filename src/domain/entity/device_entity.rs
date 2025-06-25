use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DeviceEntity {
    pub id: i32,
    pub address_id: i32,
    pub access_link: String,
    pub title: String
}
