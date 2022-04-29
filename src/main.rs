mod actions;
mod apis;
mod commands;
mod models;
mod patterns;
mod requests;
mod utils;

use crate::models::events::system::Ping;
use actions::message::handle_message_created;
use actions::system::handle_ping;
use axum::{extract::Json, routing::any, Router};
use http::{HeaderMap, StatusCode};
use models::events::message::MessageCreated;
use serde_json::{from_value, Value};
use std::env;
use std::net::SocketAddr;
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
async fn handle_event(event: &str, Json(body): Json<Value>) -> StatusCode {
    match event {
        "PING" => handle_ping(from_value::<Ping>(body).unwrap()),
        "MESSAGE_CREATED" | "DIRECT_MESSAGE_CREATED" => {
            handle_message_created(from_value::<MessageCreated>(body).unwrap()).await
        }
        _ => StatusCode::NOT_IMPLEMENTED,
    }
}
