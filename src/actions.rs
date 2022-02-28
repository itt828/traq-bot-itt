use crate::requests::post_to_traq;
use axum::Json;
use http::StatusCode;
use serde_json::Value;

pub fn handle_ping(body: Json<Value>) -> StatusCode {
    println!("[{}] PING", body["eventTime"]);
    StatusCode::NO_CONTENT
}

pub async fn handle_message_created(body: Json<Value>) -> StatusCode {
    println!(
        "[{}] MESSAGE_CREATED from: {}, content: {}, channel_id: {}, bot: {}",
        body["eventTime"],
        body["message"]["id"],
        body["message"]["plainText"],
        body["message"]["channelId"],
        body["message"]["user"]["bot"],
    );
    if body["message"]["user"]["bot"] == false {
        post_to_traq(
            body["message"]["plainText"].to_string(),
            body["message"]["channelId"].to_string(),
        )
        .await;
    }
    StatusCode::NO_CONTENT
}
