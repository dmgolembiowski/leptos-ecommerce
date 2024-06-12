//! A collection of shared type definitions and utilities
//! for the ecommerce platform.
mod models;

pub use models::{bill::*, customer::*, inventory::*, types::*};

use cfg_if::cfg_if;

#[rustfmt::skip]
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use ::errors::EcommerceAppError;
        use rusqlite::Connection;
        use leptos::prelude::*;
        use std::sync::Arc;
        use tokio::sync::Mutex;

          // Get the DB Connection from Leptos Context
        pub async fn conn() -> Result<Arc<Mutex<Connection>>, EcommerceAppError >{
            if cfg!(test) {
                use std::ptr::null_mut;
                use std::sync::atomic::{AtomicPtr, Ordering::{Release, Acquire}};
                
                static PTR: AtomicPtr<Arc<Mutex<Connection>>> = AtomicPtr::new(null_mut());

                let mut p = PTR.load(Acquire);

                if p.is_null() {
                    p = Box::into_raw(Box::new({
                        use refinery::embed_migrations;
                        
                        embed_migrations!("./../server/migrations");

                        let conn = Arc::new(Mutex::new(
                            Connection::open("./../db.db3")
                                .expect("Failed to connect to DB")
    )
                        );

                        conn
                    }));
                    if let Err(e) = PTR.compare_exchange(null_mut(), p, Release, Acquire) {
                        // Safety: p comes from Box::into_raw right above,
                        // and if it was shared with any other thread, its 
                        // contents should be thread safe - even across awaits.
                        drop(unsafe { Box::from_raw(p) });
                        p = e;
                    }
                }
                Ok(Arc::clone(&*(unsafe { &*p })))

            } else {
                let raw_conn = match use_context::<Arc<Mutex<Connection>>>(){
                    Some(c) => c.clone(),
                    None => return Err(EcommerceAppError::InternalServerError)
                };
                Ok(raw_conn)
            }
        }
    }
}
