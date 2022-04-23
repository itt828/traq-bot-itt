use crate::{apis::util::request, models::apis::message::GetMessage};
use hyper::Method;
use serde_json::json;
pub async fn get_message(message_id: &String) -> GetMessage {
    let url = format!("https://q.trap.jp/api/v3/messages/{}", message_id);
    let body = json!({});
    let resp = request(&url, Method::GET, body, "get message").await;
    serde_json::from_slice(&hyper::body::to_bytes(resp.into_body()).await.unwrap()).unwrap()
}
pub async fn post_message(content: &String, channel_id: &String) {
    let url = format!("https://q.trap.jp/api/v3/channels/{}/messages", channel_id);
    let body = json!({
      "content": content.to_string(),
      "embed": true
    });
    let _resp = request(&url, Method::POST, body, "post message").await;
}
pub async fn post_stamp(message_id: &String, stamp_id: &String, count: u64) {
    let url = format!(
        "https://q.trap.jp/api/v3/messages/{}/stamps/{}",
        message_id, stamp_id
    );
    let body = json!({
      "count": count,
    });
    let _resp = request(&url, Method::POST, body, "post stamp").await;
}
