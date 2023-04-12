use std::net::SocketAddr;

use axum;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");
    Html("Hello, Axum!")
}
