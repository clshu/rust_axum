#![allow(unused)]
use std::net::SocketAddr;

use axum::response::Html;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!<strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
