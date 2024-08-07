use crate::state::AppState;
use app::pages::App;
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
pub mod fileserv;
pub mod state;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let conn = Arc::new(Mutex::new(
        Connection::open("./db.db3").expect("Failed to connect to DB"),
    ));
    {
        let db_conn = &mut *conn.lock().await;
        embedded::migrations::runner().run(db_conn).unwrap();
    }
    let state = AppState {
        leptos_options: leptos_options.clone(),
        routes: routes.clone(),
        conn: conn.clone(),
    };
    // build our application with a route
    // dbg!(&state);
    let app = Router::new()
        .leptos_routes_with_context(
            &state,
            routes,
            move || provide_context(conn.clone()),
            move || {
                use leptos::prelude::*;

                view! {
                    <!DOCTYPE html>
                    <html lang="en">
                        <head>
                            <meta charset="utf-8"/>
                            <meta name="viewport" content="width=device-width, initial-scale=1"/>
                            // <AutoReload options=app_state.leptos_options.clone() />
                            <HydrationScripts options=leptos_options.clone()/>
                            <MetaTags/>
                        </head>
                        <body>
                            <App/>
                        </body>
                    </html>
                }
            },
        )
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
