use std::net::SocketAddr;

use axum;
use axum::extract::Path;
use axum::extract::Query;
use axum::middleware;
use axum::response::Html;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .layer(middleware::map_response(main_response_mapper))
        .merge(web::routes_login::routes());
    // TODO: Fallback

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

// middleware
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

// region: Handler
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Query extractor allows us to get parameter from query i.e. /hello?name=Jane
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, {name}!"))
}

// Path extractor allows us to get parameter from path i.e. /hello/{name}
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2", "HANDLER");

    Html(format!("Hello, {name}!"))
}
