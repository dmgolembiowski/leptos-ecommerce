use crate::pages::catalog::Catalog;
use crate::pages::catalog::InventoryItem;
use crate::pages::home::__Home;
use leptos::prelude::*;
use leptos::*;
use leptos::{component, view, IntoView};
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;
/*
    components::{Route, Router, Routes},
    StaticSegment,
};*/
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
                <Routes fallback>
                    <Route path="/catalog" view=Catalog />
                    <Route path="/catalog/:id" view=InventoryItem />
                    <Route path="" view=__Home />
                </Routes>
            </main>
        </Router>
    }
}

#[component(transparent)]
fn RootRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route path=StaticSegment("") view=__Home/>
            <Route path=StaticSegment("/") view=__Home/>
            <Route path=StaticSegment("/index.html") view=__Home/>
        <Routes/>
    }
}

#[component]
fn FullCatalog() -> impl IntoView {
    view! {
        <div></div>
    }
}
