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
                        <div style="display: grid; grid-template-columns: auto auto auto; padding: 10px;">
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
                                                        <div class="card-body" style="display: flex; padding: 20px; text-align: center;">
                                                            <img src={format!("/assets/{:}", r.asset.into_os_string().into_string().unwrap())} style="display: block; max-width: 32vh; max-height: 32vh; height: auto; width: auto;" />
                                                            <h5 class="typography">{r.name}</h5>
                                                        <br />
                                                            <div class="typography"><span class="bnb2">{r.cost}</span></div>
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
