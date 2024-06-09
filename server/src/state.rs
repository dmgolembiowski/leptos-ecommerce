#![allow(unused_imports, unused_variables)]
use axum::extract::FromRef;
use common::{
    BillLineItems, BillRow, BillRowId, Count, CustomerId, CustomerRow, Inventory, InventoryName,
    InventoryRow, InventoryRowId, Money, Time,
};
use leptos::{html::q, prelude::LeptosOptions};
use leptos_axum::AxumRouteListing;
use rusqlite::Row;
use rusqlite::{prepare_and_bind, Connection};
use std::vec::IntoIter;
use std::{borrow::Borrow, collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::{Iter as StreamIter, StreamExt, StreamMap};

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub conn: Arc<Mutex<Connection>>,
    pub routes: Vec<AxumRouteListing>,
}

impl Inventory for AppState {
    type LineItem = InventoryRow;
    type Order = BillLineItems;
    type Purchaser = CustomerId;

    async fn catalog(&self) -> StreamMap<InventoryRowId, StreamIter<IntoIter<Self::LineItem>>> {
        let conn = &mut *self.conn.lock().await;
        let mut map = StreamMap::new();
        let stmt = conn.prepare("SELECT id, name, asset, cost, quantity_available, created_at, updated_at FROM inventory;");
        let rows = {
            let mut it: Vec<InventoryRow> = vec![];
            let _ = stmt.unwrap().query_map([], |row| {
                let entry = common::InventoryRowBuilder::default()
                    .id(row.get(0)?)
                    .name(row.get(1)?)
                    .asset(<String as Into<std::path::PathBuf>>::into(row.get(2)?))
                    .cost(row.get(3)?)
                    .quantity_available(row.get(4)?)
                    .created_at(row.get(5)?)
                    .updated_at(row.get(6)?)
                    .build()
                    .unwrap();
                it.push(entry);
                Ok(())
            });
            it
        };
        let stream = tokio_stream::iter(rows);
        map.insert(0, stream);
        map
    }

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
            "INSERT INTO bills (customer_id, line_items, total) VALUES (?1, ?2)",
            params,
        )?;

        Ok(())
    }
}
