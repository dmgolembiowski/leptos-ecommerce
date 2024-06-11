use leptos::{component, IntoView, view};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
use crate::pages::home::__Home;
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
                    <Route path=StaticSegment("") view=__Home/>
                </FlatRoutes>
            </main>
        </Router>
    }
}