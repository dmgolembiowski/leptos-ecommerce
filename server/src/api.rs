use crate::state::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
    Json,
};
use log::info;

use common::{Inventory, InventoryRow};
use leptos::*;
use tokio_stream::StreamExt;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn api_handler(State(state): State<AppState>, req: Request<Body>) -> AxumResponse {
    info!("api_handler: {}", req.uri().path());
    let uri = req.uri();
    let path = uri.path();

    match path {
        "/api/catalog" => {
            let cat = <AppState as Inventory>::catalog(&state).await;
            let it = cat
                .fold(Vec::<serde_json::Value>::new(), |mut acc, (_, v)| {
                    let InventoryRow {
                        id,
                        name,
                        asset,
                        cost,
                        quantity_available,
                        ..
                    } = v;

                    acc.push(serde_json::json!({
                        "id": id,
                        "name": name,
                        "asset": asset,
                        "cost": cost,
                        "quantityAvailable": quantity_available
                    }));
                    acc
                })
                .await
                .into_iter()
                .collect::<Vec<_>>();

            Json(serde_json::to_string(&it).unwrap()).into_response()
        }
        _ => {
            panic!()
        }
    }
}
