use std::net::SocketAddr;

use crate::error::Result;
use crate::store::model::ModelController;
use axum;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Router};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;

mod error;
mod store;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        // Note: .layer gets executed from bottom to top -> If you want cookie data to be accessible by other layers,
        // you need to put it at the bottom.
        .merge(web::routes_tickets::routes(mc.clone()))
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
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
