#![allow(non_snake_case)]
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_reactive::{create_signal, *};
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
        <Catalog/>
    }
}

#[component]
pub fn Catalog() -> impl IntoView {
    let inventory_row_items = create_resource(move || functions::inventory::catalog::get_catalog());
    let (pending, set_pending) = create_signal(false);
    let hide_more_cookies = move || {
        pending.get()
            || inventory_row_items
                .map(|cookie_vec| {
                    cookie_vec
                        .as_ref()
                        .map(|it| it.len() < 34)
                        .unwrap_or_default()
                })
                .unwrap_or_default()
    };
    view! {
        {move || if hide_more_cookies() {
             view! {
                <div> "Loading..." </div>
             }
        }
        else {
            view! {
                <div>
                    <Transition
                        fallback=|| ()
                        set_pending
                    >
                        {move || inventory_row_items.get().map(|cookie_row| cookie_row.map(|row| view! {
                            <ul>
                                <For
                                    each=move || row.clone()
                                    key=|r| r.id
                                    let:r
                                >
                                    <li>
                                        <span>{r.name}</span>
                                        <span>{r.price}</span>
                                        <span>{r.asset}</span>
                                        <span>{r.cost}</span>
                                        <span>{r.quantity_available}</span>
                                    </li>
                                </For>
                            </ul>
                        }))}
                    </Transition>
                </div>
            }
        }}
    }
}
