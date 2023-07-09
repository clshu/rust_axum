#![allow(unused)]
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let routes_all = Router::new().merge(routes_hello());
    // Create socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}
// Routers
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// Handlers
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g. /hello?name=Jane
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params
        .name
        .as_deref()
        .unwrap_or("World")
        .trim()
        .trim_matches('"');

    Html(format!("Hello <strong>{name}!!!<strong>"))
}
// e.g. /hello2/Mike
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2", "HANDLER");

    Html(format!("Hello2 <strong>{name}!!!<strong>"))
}
