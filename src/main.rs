mod actions;
mod requests;
mod utils;
use actions::*;
use axum::{extract::Json, routing::any, Router};
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
mod models;
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new().route("/", any(handler));
    println!("serving at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(body: Json<Value>, headers: HeaderMap) -> StatusCode {
    let token = headers["X-TRAQ-BOT-TOKEN"].to_str();
    if token.is_err() {
        println!("No X-TRAQ-BOT-TOKEN");
        return StatusCode::BAD_REQUEST;
    }
    let token = token.unwrap().to_string();
    let event = headers["X-TRAQ-BOT-EVENT"].to_str();
    if event.is_err() {
        println!("No X-TRAQ-EVENT");
        return StatusCode::BAD_REQUEST;
    }
    let event = event.unwrap();
    if token == env::var("BOT_VERIFICATION_TOKEN").expect("BOT_VERIFICATION_TOKEN not found") {
        return handle_event(event, body).await;
    } else {
        return StatusCode::UNAUTHORIZED;
    }
}
async fn handle_event(event: &str, body: Json<Value>) -> StatusCode {
    match event {
        "PING" => handle_ping(body),
        "MESSAGE_CREATED" => handle_message_created(body).await,
        _ => StatusCode::NOT_IMPLEMENTED,
    }
}
