use crate::functions::inventory::get_catalog;
use leptos::prelude::{ElementChild, *};
#[component]
pub fn Catalog() -> impl IntoView {
    let inventory_row_items =
        Resource::new_serde(|| (), move |_| async move { get_catalog().await });
    {
        move || {
            view! {
                <div>
                    <Transition fallback=|| {
                        view! { <p>"Loading"</p> }
                    }>
                        <div class="card-product col-gallery" style="display: grid; grid-template-columns: auto auto auto auto; justify-content: ; align-content: center;">
                        {move || {
                            inventory_row_items
                                .get()
                                .map(|cookie_row| {
                                    cookie_row
                                        .map(|row_vec| {
                                            row_vec
                                                .into_iter()
                                                .map(|r| {
                                                    view! {
                                                        <div class="card-body pswipe-gallery" style="padding: 20px; justify-content: end;">
                                                            <img src={format!("/assets/{:}", r.asset.into_os_string().into_string().unwrap())} style="display: block; max-width: 32vh; max-height: 32vh; height: auto; width: auto; align-content: space-around;" />
                                                            <h5 class="typography">{r.name}</h5>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        })
                                })
                        }}
                        </div>

                    </Transition>
                </div>
            }
        }
    }
}

#[component]
pub fn Card(public_path: String) -> impl IntoView {
    view! {
        <img src={public_path} />
    }
}
