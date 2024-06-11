use crate::pages::Catalog;
use leptos::prelude::{signal, ElementChild, OnAttribute, Update};
use leptos::{component, view, IntoView};
/// Renders the home page of your application.
#[component]
fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    //  <button on:click=on_click>"Click Me: " {count}</button>
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Catalog/>
    }
}

