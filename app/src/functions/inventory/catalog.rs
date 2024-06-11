use ::errors::EcommerceAppError;
use common::InventoryRow;
use leptos::prelude::*;

#[server(GetCatalog, "/api/catalog")]
pub async fn get_catalog() -> Result<Vec<InventoryRow>, ServerFnError<EcommerceAppError>> {
    use common::Inventory;

    Ok(Inventory::get()
        .await
        .map_err(|e| ServerFnError::WrappedServerError(e))
        .and_then(|inv: Inventory| Ok(inv.into_inner()))?)
}
