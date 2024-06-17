use crate::components::{card::Card, grid::Grid};
use crate::functions::inventory::get_catalog;
use crate::functions::inventory::get_inventory_item;
use hooks::use_params_map;
use leptos::prelude::{ElementChild, *};
use leptos::*;
use leptos_meta::*;
use leptos_router::hooks::use_params;
use leptos_router::*;
// use leptos_router::*;
use common::InventoryRow;
use common::InventoryRowId;
use leptos::component;
use params::ParamsMap;

#[rustfmt::skip]
#[component]
pub fn Catalog() -> impl IntoView {
    let inventory_row_items =
        Resource::new_serde(|| (), move |_| async move { get_catalog().await });
    { move || {
    view! {
        <div>
            <Transition fallback=|| { view! { <p>"Loading"</p> } }>
            <Grid> {move || {
                inventory_row_items
                .get()
                .map(|cookie_row| { cookie_row.map(|row_vec| { row_vec.into_iter().map(|r| {
                    let p = r.asset.into_os_string().into_string().unwrap();
                    let public_path = format!("/assets/{:}", p);
                    view! {
                        <Card public_path={public_path} name={r.name} />
                    }
                }).collect_view()
                })})
            }}
            </Grid>
            </Transition>
        </div>
    }}}
}

// show_id receives the read signal from the page component, which is responsible for
// setting the id of the corresponding inventory item to be displayed.
// here, we create a resource to call Inventory::get_inventory_item with the id.
#[component]
pub fn InventoryItem() -> impl IntoView {
    // The purpose behind this dummy id is to dodge `<u32 as Default>::default()` being 0.
    // We arbitrarily set some big number that should exceed the IDs of the inventory row data.
    const DUMMY_ID: u32 = u32::MAX;

    let params: Memo<ParamsMap> = use_params_map();

    let get_id = move || {
        params.with(|params: &ParamsMap| {
            params
                .get("id")
                .unwrap_or(DUMMY_ID.to_string())
                .parse::<u32>()
                .unwrap()
        })
    };

    let item = Resource::new(get_id, move |_| async move {
        get_inventory_item(get_id()).await.unwrap()
    });

    view! {
        <div>
            <Transition fallback=|| { view! { <p>"Loading"</p> } }>
            {move || {
                if let Some(InventoryRow { id: _, name, asset, .. }) = item.get() {
                    let p = asset.into_os_string().into_string().unwrap();
                    let public_path = format!("/assets/{:}", p);
                    view! {
                        <Card public_path={public_path} name={name} />
                    }
                } else {
                    view! {
                        <Card public_path="/assets/placeholder.jpg".to_string() name="Cookie Not Found".to_string() />
                    }
                }
            }}
            </Transition>
        </div>
    }
}
