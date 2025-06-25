use serde::Serialize;
use sqlx::FromRow;

use crate::domain::entity::address_entity::AddressEntity;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DeviceEntity {
    pub id: i32,
    pub address_id: i32,
    pub address_entity: AddressEntity,
    pub access_link: String,
    pub title: String
}
