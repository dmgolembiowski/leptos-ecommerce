use crate::pages::home::__Home;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};
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
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width,inistial-scale=1.0" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <Link href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;600;700;800&display=swap" rel="stylesheet" />
        <Stylesheet id="leptos" href="/pkg/leptos-ecommerce.css"/>

        // sets the document title
        <Title text="Browser Bakery"/>

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
