use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub conn: Arc<Mutex<Connection>>,
    pub routes: Vec<AxumRouteListing>,
}
