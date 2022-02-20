use axum::{extract::Json, routing::any, Router};
use http::{HeaderMap, StatusCode};
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::{json, Value};
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
    if is_token_valid(token) {
        return handle_event(event, body).await;
    } else {
        return StatusCode::UNAUTHORIZED;
    }
}
async fn handle_event(event: &str, body: Json<Value>) -> StatusCode {
    if event == "PING" {
        println!("PING");
        return StatusCode::NO_CONTENT;
    } else if event == "MESSAGE_CREATED" {
        handle_message_created(body).await;
        return StatusCode::NO_CONTENT;
    }
    StatusCode::NOT_IMPLEMENTED
}

async fn handle_message_created(body: Json<Value>) {
    let text = body["message"]["plainText"].to_string();
    let text = remove_double_quotes(text);
    let channel_id = body["message"]["channelId"].to_string();
    let channel_id = remove_double_quotes(channel_id);
    println!("text = {}, channel_id = {}", text, channel_id);
    if body["message"]["user"]["bot"] == true {
        println!("message from bot");
    } else {
        post_to_traq(text, channel_id).await;
    }
}
fn remove_double_quotes(string: String) -> String {
    let le = string.len();
    string[1..le - 1].to_string()
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
        .body(Body::from(
            json!({
              "content": text,
              "embed": true
            })
            .to_string(),
        ))
        .unwrap();
    println!("{:?}", req);
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let resp = client.request(req).await.unwrap();
    println!("Response: {}", resp.status());
}
