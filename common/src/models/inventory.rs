use crate::models::types::*;
use derive_builder::Builder;
use std::path::PathBuf;

/// By default, the server will use this type
/// to implement the `Inventory` trait, however
/// this can be overridden to help with orphan rules.
#[derive(Default, Builder, Debug, Clone)]
#[builder]
pub struct InventoryRow {
    pub id: InventoryRowId,
    pub name: String,
    pub asset: PathBuf,
    pub cost: Money,
    pub quantity_available: Count,
    pub created_at: Time,
    pub updated_at: Time,
}
