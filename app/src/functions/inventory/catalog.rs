use leptos::prelude::*;
use server::AppStore;

#[server]
pub async fn get_catalog() -> Result<(), ServerFnError>{

    AppStore::catalog();
    Ok(())
}
