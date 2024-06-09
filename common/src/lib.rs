//! A collection of shared type definitions and utilities
//! for the ecommerce platform.
mod models;
pub use models::{bill::*, customer::*, inventory::*, types::*};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use errors::AppError;

        use rusqlite::Connection
        use leptos::prelude::*;

          // Get the DB Connection from Leptos Context
        pub async fn conn() -> Result<Connection, AppError >{
            use_context::<AppState>().ok_or("Conn missing").map_err(|_| AppError::InternalServerError)
        }
}}






