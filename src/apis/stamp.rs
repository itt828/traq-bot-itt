use super::util::request;
use crate::models::apis::stamp::Stamps;
use hyper::Method;
use serde_json::json;
pub async fn get_stamps() -> Stamps {
    let url = format!("https://q.trap.jp/api/v3/stamps?include-unicode=true");
    let body = json!({});
    let resp = request(&url, Method::GET, body, "get stamps").await;
    serde_json::from_slice(&hyper::body::to_bytes(resp.into_body()).await.unwrap()).unwrap()
}
