use leptos::children::*;
use leptos::prelude::*;
use leptos::{html::*, *};

#[component]
pub fn Grid(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class={"grid-l"}>
            {children()}
        </div>
    }
}
