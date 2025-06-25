use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AddressEntity {
    pub id: i32,
    pub region_id: i32,
    pub region: NamedSubAddressEntity,
    pub city_id: i32,
    pub city: NamedSubAddressEntity,
    pub street_id: i32,
    pub street: NamedSubAddressEntity,
    pub house_id:i32,
    pub house: NamedSubAddressEntity,
    pub entrance_id: i32,
    pub entrance: DescriptiveNumerableSubAddressEntity,
    pub floor_id: i32,
    pub floor: NamedSubAddressEntity,
    pub flat_id: i32,
    pub flat: NamedSubAddressEntity
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct NamedSubAddressEntity {
    pub id: u32,
    pub value: String
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DescriptiveNumerableSubAddressEntity {
    pub id: u32,
    pub value: String,
    pub description: String
}