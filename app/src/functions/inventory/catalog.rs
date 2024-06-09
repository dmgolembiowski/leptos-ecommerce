use leptos::prelude::*;
use common::Inventory;
use ::errors::EcommerceAppError;
#[server]
pub async fn get_catalog() -> Result<(), ServerFnError<EcommerceAppError>>{

    Inventory::get();
    Ok(())
}
