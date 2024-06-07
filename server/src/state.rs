#![allow(unused_imports, unused_variables)]
use axum::extract::FromRef;
use common::{
    inventory::Inventory, BillRow, BillRowId, Count, CustomerId, CustomerRow, InventoryRow,
    InventoryRowId, Money, Time,
};
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::{StreamExt, StreamMap};

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
    type Order = BillRow;

    async fn catalog(
        &self,
    ) -> StreamMap<InventoryRowId, tokio_stream::Iter<std::vec::IntoIter<InventoryRow>>> {
        let conn = self.conn.lock().await;
        let mut map = StreamMap::new();
        let stmt = conn.prepare("SELECT id, name, asset, cost, quantity_available, created_at, updated_at FROM inventory;");
        let rows = {
            let mut it: Vec<InventoryRow> = vec![];
            let _ = stmt.unwrap().query_map([], |row| {
                let id: InventoryRowId = row.get(0)?;
                let name: String = row.get(1)?;
                let asset: String = row.get(2)?;
                let cost: Money = row.get(3)?;
                let quantity_available: Count = row.get(4)?;
                let created_at: Time = row.get(5)?;
                let updated_at: Time = row.get(6)?;
                it.push(InventoryRow {
                    id,
                    name,
                    asset: asset.into(),
                    cost,
                    quantity_available,
                    created_at,
                    updated_at,
                });
                Ok(())
            });
            it
        };
        let stream = tokio_stream::iter(rows);
        map.insert(0, stream);
        map
    }

    async fn purchase(&self, order: &Self::Order) -> Result<(), Box<dyn std::error::Error>> {
        Err("Not implemented".into())
    }
}
