#![allow(unused_import_braces, unused_imports)]
#[rustfmt::skip]
use {
    axum::extract::FromRef,
    common::{
        BillLineItems, 
        Count, 
        CustomerId, 
        Inventory, 
        InventoryName,
        InventoryRow, 
        InventoryRowId, 
        Money, 
    },
    leptos::{
        html::q, 
        prelude::LeptosOptions,
    },
    leptos_axum::AxumRouteListing,
    rusqlite::Connection,
    std::{
        collections::BTreeMap, 
        sync::Arc, 
        vec::IntoIter,
    },
    tokio::sync::Mutex,
    tokio_stream::{
        Iter as StreamIter, 
        StreamExt, 
        StreamMap,
    },
};
/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub conn: Arc<Mutex<Connection>>,
    pub routes: Vec<AxumRouteListing>,
}
