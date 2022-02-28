use axum::Json;
use http::StatusCode;
use serde_json::Value;

pub fn handle_ping(body: Json<Value>) -> StatusCode {
    println!("[{}] PING", body["eventTime"]);
    StatusCode::NO_CONTENT
}
