use crate::models::types::*;
use cfg_if::cfg_if;
use derive_builder::Builder;
use errors::EcommerceAppError;
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[repr(transparent)]
pub struct Inventory(Vec<InventoryRow>);

impl Inventory {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use crate::conn;

            impl Inventory{
                    pub async fn get() -> Result<Self, EcommerceAppError> {
                    let conn_raw = conn().await?;
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