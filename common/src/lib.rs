//! A collection of shared type definitions and utilities
//! for the ecommerce platform.
mod models;
pub use models::{bill::*, customer::*, inventory::*, types::*};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use ::errors::EcommerceAppError;
        use leptos_axum::extract_with_state;
        use rusqlite::Connection;
        use leptos::prelude::*;
        use axum::extract::State;
        use std::sync::Arc;
        use tokio::sync::Mutex;

          // Get the DB Connection from Leptos Context
        pub async fn conn() -> Result<Arc<Mutex<Connection>>, EcommerceAppError >{
            let raw_conn = match use_context::<Arc<Mutex<Connection>>>(){
                Some(c) => c.clone(),
                None => return Err(EcommerceAppError::InternalServerError)
            };
            Ok(raw_conn)
        }
}}





