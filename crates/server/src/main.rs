mod log;
use std::net::SocketAddr;

use axum::{response::Html, routing, Router};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    log::initialise();

    let app = Router::new()
        .route("/", routing::get(handler))
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {address}");

    if let Err(e) = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
    {
        error!("{e}")
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Rust</h1>")
}
