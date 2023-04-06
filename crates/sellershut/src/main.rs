use std::net::SocketAddr;

use axum::{response::Html, routing, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(handler));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {address}");

    if let Err(e) = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("{e}")
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Rust</h1>")
}
