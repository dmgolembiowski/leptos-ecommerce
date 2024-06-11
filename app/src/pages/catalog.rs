use leptos::prelude::{*, ElementChild};
use crate::functions::inventory::get_catalog;
#[component]
pub fn Catalog() -> impl IntoView {
    let inventory_row_items = Resource::new_serde(
        || (),
        move |_| async move {get_catalog().await},
    );
        {move || {
                     view! {
                         <div>
                             <Transition fallback=|| {
                                 view! { <p>"Loading"</p> }
                             }>
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
                                                                 <ul>
                                                                     <li>
                                                                         <span>{r.name}</span>
                                                                         <span>{r.cost}</span>
                                                                         <span>
                                                                             {r.asset.into_os_string().into_string().unwrap()}
                                                                         </span>
                                                                         <span>{r.quantity_available}</span>
                                                                     </li>
                                                                 </ul>
                                                             }
                                                         })
                                                         .collect_view()
                                                 })
                                         })
                                 }}

                             </Transition>
                         </div>
                     }
            }
        }}

