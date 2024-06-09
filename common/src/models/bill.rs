use crate::models::types::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Builder)]
#[builder]
pub struct BillRow {
    pub id: BillRowId,
    pub customer_id: CustomerId,
    pub line_items: BillLineItems,
    pub total: Money,
    pub created_at: Time,
    pub updated_at: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BillLineItems(pub BTreeMap<InventoryName, Count>);

impl std::fmt::Display for BillLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use serde_json;
        write!(f, "{}", serde_json::to_string(&self.0).unwrap())
    }
}

impl TryFrom<&str> for BillLineItems {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

impl BillLineItems {
    pub fn new(inner: BTreeMap<InventoryName, Count>) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> BTreeMap<InventoryName, Count> {
        self.0
    }

    pub fn dumps(&self) -> String {
        self.to_string()
    }

    pub fn loads(s: &str) -> Result<Self, serde_json::Error> {
        <Self as TryFrom<&str>>::try_from(s)
    }
}

impl Iterator for BillLineItems {
    type Item = (InventoryName, Count);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().map(|(k, v)| (k.clone(), v.clone()))
    }
}

pub struct Bill(BillRow);

impl Bill {
    /*
    /// This generates a database record and updates the inventory in response
    /// to a requested cart's order.
    async fn purchase(
        &self,
        order: &Self::Order,
        cust: &Self::Purchaser,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cnx = &mut *self.conn.lock().await;
        let can_proceed = &mut true;

        for (inv_name, count) in order.clone() {
            let mut stmt = cnx
                .prepare("SELECT quantity_available, name FROM inventory WHERE (name = ?1)")
                .unwrap();
            let left = stmt.query_row([inv_name], |row| row.get(1)).unwrap();
            if count > left {
                *can_proceed = false;
                break;
            }
        }

        if !*can_proceed {
            return Err("Insufficient inventory".into());
        }

        let mut total_cost: Money = 0.00;
        let mut line_items = BTreeMap::<InventoryName, Count>::new();

        type Data = Result<(u32, String, f32), rusqlite::Error>;

        for (inv_name, count) in order.clone() {
            let data: Data = cnx.query_row_and_then(
                "SELECT (id, cost, name) FROM inventory WHERE (name = :inv_name)",
                &[(":inv_name", &inv_name)],
                |row| {
                    Ok((
                        row.get::<_, u32>(0).unwrap(),
                        row.get::<_, String>(1).unwrap(),
                        row.get::<_, f32>(2).unwrap(),
                    ))
                },
            );
            let (inv_id, inv_name, cost) = data?;
            total_cost += cost * count as Money;
            line_items.insert(inv_name, count);

            let mut stmt = cnx.prepare(
                "UPDATE inventory SET quantity_available = quantity_available - ?1 WHERE (id = ?2)",
            )?;
            stmt.execute(rusqlite::params![count, inv_id])?;
        }

        let li = serde_json::to_string(&line_items)?;
        let params = rusqlite::params![cust, li, total_cost];

        cnx.execute(
            "INSERT INTO bills (customer_id, line_items, total) VALUES (?1, ?2, ?3)",
            params,
        )?;
        Ok(())
    }
    */
}
