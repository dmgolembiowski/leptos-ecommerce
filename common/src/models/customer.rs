use crate::models::types::*;
use derive_builder::Builder;

#[derive(Builder, Debug, Clone)]
#[builder]
pub struct CustomerRow {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub created_at: Time,
    pub updated_at: Time,
}
