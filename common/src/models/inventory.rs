use crate::models::types::*;
use derive_builder::Builder;
use std::path::PathBuf;
use cfg_if::cfg_if;
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

pub struct Inventory(Vec<InventoryRow>);

cfg_if! {
if #[cfg(feature = "ssr")] {
        use app::functions::conn;
impl Inventory{
        async fn get(&self) -> Self {
        let conn_raw = conn();
        let conn = &mut *conn_raw.lock().await;
        let mut stmt = conn.prepare("SELECT id, name, asset, cost, quantity_available, created_at, updated_at FROM inventory").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut it: Vec<InventoryRow> = vec![];
        while let Some(row) = rows.next().unwrap() {
            it.push(
                common::InventoryRowBuilder::default()
                    .id(row.get(0).unwrap())
                    .name(row.get(1).unwrap())
                    .asset(<String as Into<std::path::PathBuf>>::into(
                        row.get(2).unwrap(),
                    ))
                    .cost(row.get(3).unwrap())
                    .quantity_available(row.get(4).unwrap())
                    .created_at(row.get(5).unwrap())
                    .updated_at(row.get(6).unwrap())
                    .build()
                    .unwrap(),
            );
        }
        Inventory(it)
    }


}
}
    }