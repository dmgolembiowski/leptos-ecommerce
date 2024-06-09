#![allow(non_snake_case)]

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
pub mod error_template;
pub mod functions;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let fallback = || {
        {
            view! { <p>"Error"</p> }
        }
        .into_view()
    };
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-ecommerce.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <FlatRoutes fallback>
                    <Route path=StaticSegment("") view=HomePage/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    //  <button on:click=on_click>"Click Me: " {count}</button>
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
