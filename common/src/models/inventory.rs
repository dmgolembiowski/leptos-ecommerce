use crate::models::types::*;
use cfg_if::cfg_if;
use derive_builder::Builder;
use errors::EcommerceAppError;
use std::fmt;
use std::path::PathBuf;

/// By default, the server will use this type
/// to implement the `Inventory` trait, however
/// this can be overridden to help with orphan rules.
#[derive(serde::Serialize, serde::Deserialize, Default, Builder, Debug, Clone)]
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

impl fmt::Display for InventoryRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", &s)
    }
}

impl std::str::FromStr for InventoryRow {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[repr(transparent)]
pub struct Inventory(Vec<InventoryRow>);

impl Inventory {
    pub fn rows(&self) -> &Vec<InventoryRow> {
        &self.0
    }
    pub fn into_inner(self) -> Vec<InventoryRow> {
        self.0
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        impl Inventory{
            pub async fn get() -> Result<Self, EcommerceAppError> {
                let conn_raw = crate::conn().await?;
                let conn = &mut *conn_raw.lock().await;
                let mut stmt = conn.prepare("SELECT id, name, asset, cost, quantity_available, created_at, updated_at FROM inventory").unwrap();
                let mut rows = stmt.query([]).unwrap();
                let mut it: Vec<InventoryRow> = vec![];
                while let Some(row) = rows.next().unwrap() {
                    it.push(
                        crate::InventoryRowBuilder::default()
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
                Ok(Inventory(it))
            }

            pub async fn get_inventory_item(id: InventoryRowId) -> Result<InventoryRow, rusqlite::Error> {
                let conn_raw = crate::conn().await.map_err(|e| rusqlite::Error::InvalidPath("db.db3".to_string().into()))?;
                let conn = &mut *conn_raw.lock().await;
                conn.query_row("
                SELECT 
                    id, name, asset, cost, quantity_available, created_at, updated_at 
                FROM 
                    inventory 
                WHERE id = ?1", [id], |row| 
                Ok(InventoryRowBuilder::default()
                    .id(row.get(0).unwrap())
                    .name(row.get(1).unwrap())
                    .asset(<String as Into<std::path::PathBuf>>::into(row.get(2).unwrap()))
                    .cost(row.get(3).unwrap())
                    .quantity_available(row.get(4).unwrap())
                    .created_at(row.get(5).unwrap())
                    .updated_at(row.get(6).unwrap())
                    .build()
                    .unwrap()
                ))
            }

        }
    }
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod tests {
    use super::Inventory;
    use tokio;

    #[tokio::test]
    async fn test_inventory_retrieval() {
        let res = Inventory::get().await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.rows().len() > 0, "expected more than 0 rows");
    }
}
