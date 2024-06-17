use ::errors::EcommerceAppError;
use camino::Utf8Path;
use common::InventoryRow;
use leptos::prelude::*;
use leptos::*;

#[server(GetCatalog, "/api/catalog")]
pub async fn get_catalog() -> Result<Vec<InventoryRow>, ServerFnError<EcommerceAppError>> {
    use common::Inventory;

    Ok(Inventory::get()
        .await
        .map_err(|e| ServerFnError::WrappedServerError(e))
        .and_then(|inv: Inventory| Ok(inv.into_inner()))?)
}

#[server(GetInventoryItem, "/api/inventory/:id")]
pub async fn get_inventory_item(id: u32) -> Result<InventoryRow, ServerFnError<EcommerceAppError>> {
    use common::Inventory;

    Ok(Inventory::get_inventory_item(id).await.unwrap())
}

#[server(GetAsset, "/api/asset")]
pub async fn get_asset(asset_name: String) -> Result<(), ServerFnError> {
    use leptos_axum::redirect;
    let asset_path: String = format!("/assets/{}", &asset_name.as_str());
    Ok(redirect(asset_path.as_str()))
}
