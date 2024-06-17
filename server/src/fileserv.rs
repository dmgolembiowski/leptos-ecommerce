#![allow(unused)]

use app::pages::App;
pub use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
    routing::{get, IntoMakeService},
    Router,
};
pub use leptos::{logging::log, prelude::provide_context, prelude::LeptosOptions, view, *};
pub use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::ServiceExt;
use tower_http::services::ServeDir;

// This custom handler lets us provide Axum State via context
pub async fn custom_handler(
    Path(id): Path<String>,
    State(options): State<LeptosOptions>,
    req: Request<AxumBody>,
) -> AxumResponse {
    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(id.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<AxumBody>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(move || view! { <App/> });
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<AxumBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(AxumBody::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
