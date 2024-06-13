use crate::components::{card::Card, grid::Grid};
use crate::functions::inventory::get_catalog;
use leptos::prelude::{ElementChild, *};

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
