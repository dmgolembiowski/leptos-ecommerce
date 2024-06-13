use crate::pages::Catalog;
use leptos::prelude::{signal, ClassAttribute, ElementChild, OnAttribute, Update};
use leptos::*;
use leptos::{component, view, IntoView};
/// Renders the home page of your application.
#[component]
fn Home() -> impl IntoView {
    // Creates a reactive value to update the button

    //  <button on:click=on_click>"Click Me: " {count}</button>
    /*view! {
        <section class=("layout layout-has-sider layout-dashboard")>
            <h2>"Byte-sized Cookies"</h2>
            <Catalog />
            <div class=("layout-content")>
                <Catalog/>
            </div>
        </section>
    }
    */
    view! {
        <section>
            <div class=("logo")>
                <img src="/assets/logo-transparent.png" alt="Grandma Ben's Byte-sized Cookies" />
            </div>
            <Catalog />
        </section>
    }
}
