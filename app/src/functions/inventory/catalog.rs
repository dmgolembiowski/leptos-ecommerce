use leptos::prelude::*;
use common::Inventory;
use errors::AppError;
#[server]
pub async fn get_catalog() -> Result<(), ServerFnError<AppError>>{

    Inventory::get();
    Ok(())
}
