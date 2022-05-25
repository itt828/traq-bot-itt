use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
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
    pub async fn post_message(&self, channel_id: &str, content: &str, embed: bool) -> Result<Post> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        let body = json!({ "content": content, "embed":embed });
        let resp = self
            .api_request_base(&url, Method::POST, body)
            .await?
            .json::<Post>()
            .await?;
        Ok(resp)
    }
}
