
use crate::components::grid::Grid;
use crate::functions::inventory::get_catalog;
use leptos::prelude::{ElementChild, *};

#[component]
pub fn Card(public_path: String, name: String) -> impl IntoView {
    view! {
        <div class="grid-item">
        <img src=public_path />
        <h5>{name}</h5>
        </div>
    }
}
