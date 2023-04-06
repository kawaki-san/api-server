mod log;
use std::net::SocketAddr;

use api::{build_schema, AppSchema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing, Extension, Router,
};
use entity::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use tower_http::trace::TraceLayer;
use tracing::{error, info};

#[cfg(debug_assertions)]
use dotenvy::dotenv;

const ENDPOINT_PATH: &str = "/api/graphql";

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    log::initialise();

    let schema = build_schema().await;

    let app = Router::new()
        .route(
            ENDPOINT_PATH,
            routing::get(graphql_playground).post(handler),
        )
        .layer(Extension(schema))
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

async fn handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        ENDPOINT_PATH,
    )))
}
