#![allow(unused)]

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>Rust!!!</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()
}
