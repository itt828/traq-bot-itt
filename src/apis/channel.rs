use crate::{apis::util::request, models::apis::channel::GetChannels};
use hyper::Method;
use serde_json::json;
pub async fn get_channels() -> GetChannels {
    let url = format!("https://q.trap.jp/api/v3/channels");
    let body = json!({});
    let resp = request(&url, Method::GET, body, "get channels").await;
    serde_json::from_slice(&hyper::body::to_bytes(resp.into_body()).await.unwrap()).unwrap()
}
