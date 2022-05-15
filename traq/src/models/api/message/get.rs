use crate::bot::Bot;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get {
    id: String,
    user_id: String,
    channel_id: String,
    content: String,
    created_at: String,
    updated_at: String,
    pinned: bool,
    stamps: Vec<Stamp>,
    thread_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    user_id: String,
    stamp_id: String,
    count: String,
    created_at: String,
    updated_at: String,
}

impl Bot {
    pub async fn get_message(&self, message_id: String) -> Result<Get, Box<dyn std::error::Error>> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::GET, body)
            .await?
            .json::<Get>()
            .await?;
        Ok(resp)
    }
}
