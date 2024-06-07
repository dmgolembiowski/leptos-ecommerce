//! A collection of shared type definitions and utilities
//! for the ecommerce platform.
#![allow(async_fn_in_trait, unused_imports)]

use std::path::PathBuf;

pub mod inventory;

pub type Count = usize;
pub type InventoryRowId = u32;
pub type Time = u64;

pub type Money = f32;

pub struct InventoryRow {
    pub id: InventoryRowId,
    pub name: String,
    pub asset: PathBuf,
    pub cost: Money,
    pub quantity_available: Count,
    pub created_at: Time,
    pub updated_at: Time,
}

pub type BillRowId = u32;

pub struct BillRow {
    pub id: BillRowId,
    pub line_items: Vec<(InventoryRowId, Count)>,
    pub total: Money,
    pub created_at: Time,
    pub updated_at: Time,
}

pub type CustomerId = u32;

pub struct CustomerRow {
    pub id: CustomerId,
    pub name: String,
    pub email: String,
    pub bills: Vec<BillRowId>,
    pub created_at: Time,
    pub updated_at: Time,
}
