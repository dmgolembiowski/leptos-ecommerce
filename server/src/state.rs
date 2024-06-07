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
use tokio_stream::{StreamExt, StreamNotifyClose};

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

    async fn catalog(&self) -> StreamNotifyClose<Self::LineItem> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, name, asset, cost, quantity_available, created_at, updated_at FROM inventory;");
        let mut rows = stmt
            .unwrap()
            .query_map([], |row| {
                let id: InventoryRowId = row.get(0)?;
                let name: String = row.get(1)?;
                let asset: String = row.get(2)?;
                let cost: Money = row.get(3)?;
                let quantity_available: Count = row.get(4)?;
                let created_at: Time = row.get(5)?;
                let updated_at: Time = row.get(6)?;
                Ok(InventoryRow {
                    id,
                    name,
                    asset: asset.into(),
                    cost,
                    quantity_available,
                    created_at,
                    updated_at,
                })
            })
            .unwrap();

        let stream = rows.filter_map(|row| Some(async move { row.ok() })).boxed();
        stream
    }

    async fn purchase(&self, order: &Self::Order) -> Result<(), Box<dyn std::error::Error>> {
        Err("Not implemented".into())
    }
}
