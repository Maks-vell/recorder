use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

use crate::domain::entity::address_entity::AddressEntity;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct RecordEntity {
    pub id: i32,
    pub device_id: i32,
    pub address_id: i32,
    pub address: AddressEntity,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub path: String,
}