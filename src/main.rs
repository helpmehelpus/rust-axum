#![allow(unused)]

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use axum::extract::{Path, Query};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap()
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Rust!");
    Html(format!("Hello <strong>{name}!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}!</strong>"))
}