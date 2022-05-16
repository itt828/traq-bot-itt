mod actions;
mod commands;
mod patterns;
use crate::actions::message::handle_message_created;
use actions::system::handle_ping;
use anyhow::Result;
use axum::{extract::Json, routing::any, Router};
use http::{HeaderMap, StatusCode};
use serde_json::{from_value, Value};
use std::env;
use std::net::SocketAddr;
use traq::{bot::Bot, models::event::*};
#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot {
        base_url: "".to_string(),
        bot_access_token: "".to_string(),
    };
    let bot = std::sync::Arc::new(bot);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new().route("/", any(|body, headers| handler(bot, body, headers)));
    println!("serving at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn handler(bot: std::sync::Arc<Bot>, body: Json<Value>, headers: HeaderMap) -> StatusCode {
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
        return handle_event(bot, event, body).await;
    } else {
        return StatusCode::UNAUTHORIZED;
    }
}
async fn handle_event(
    bot: std::sync::Arc<Bot>,
    event: &str,
    Json(body): Json<Value>,
) -> StatusCode {
    match event {
        "PING" => handle_ping(from_value::<system::Ping>(body).unwrap()),
        "MESSAGE_CREATED" | "DIRECT_MESSAGE_CREATED" => handle_message_created(
            bot,
            std::sync::Arc::new(from_value::<message::MessageCreated>(body).unwrap()),
        )
        .await
        .unwrap(),
        _ => StatusCode::NOT_IMPLEMENTED,
    }
}
