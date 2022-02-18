use axum::{extract::Json, routing::any, Router};
use dotenv::dotenv;
use http::{HeaderMap, StatusCode};
use hyper::{Body, Client, Method, Request};
use serde_json::Value;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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
    if is_token_valid(token) {
        handle_event(event, body).await;
    } else {
        return StatusCode::UNAUTHORIZED;
    }
    StatusCode::BAD_REQUEST
}
async fn handle_event(event: &str, body: Json<Value>) -> StatusCode {
    if event == "PING" {
        return StatusCode::NO_CONTENT;
    } else if event == "MESSAGE_CREATED" {
        handle_message_created(body).await;
        return StatusCode::NO_CONTENT;
    }
    StatusCode::NOT_IMPLEMENTED
}

async fn handle_message_created(body: Json<Value>) {
    let text = body["message"]["plainText"].to_string();
    let channel_id = body["message"]["channelId"].to_string();
    if body["message"]["user"]["bot"].is_boolean() {
        println!("message from bot");
    }
    post_to_traq(text, channel_id).await;
}

fn is_token_valid(token: String) -> bool {
    return token == env::var("BOT_VERIFICATION_TOKEN").expect("BOT_VERIFICATION_TOKEN not found");
}

async fn post_to_traq(text: String, channel_id: String) {
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!(
            "https://q.trap.jp/api/v3/channels/{}/messages",
            channel_id
        ))
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not found")
            ),
        )
        .body(Body::from(format!(
            "{{\"content\":{},\"embed\":true}}",
            text
        )))
        .unwrap();
    let client = Client::new();
    let resp = client.request(req).await.unwrap();
    println!("Response: {}", resp.status());
}
