use super::util::request;
use hyper::Method;
use serde_json::json;
static BOT_ID: &str = "78d4d314-7b1d-4486-bbbf-65759fe77033";
pub async fn join(channel_id: &String) {
    let url = format!("https://q.trap.jp/api/v3/bots/{}/actions/join", BOT_ID);
    let body = json!({ "channelId": channel_id });
    let _resp = request(&url, Method::POST, body, "join").await;
}
pub async fn leave(channel_id: &String) {
    let url = format!("https://q.trap.jp/api/v3/bots/{}/actions/actions", BOT_ID);
    let body = json!({ "channelId": channel_id });
    let _resp = request(&url, Method::POST, body, "join").await;
}
