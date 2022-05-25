use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTopic {}

impl Bot {
    pub async fn edit_topic(&self, channel_id: &str, topic: &str) -> Result<EditTopic> {
        let url = format!("{}/channels/{}/topic", self.base_url, channel_id);
        let body = json!({ "topic": topic });
        let resp = self
            .api_request_base(&url, Method::PUT, body)
            .await?
            .json::<EditTopic>()
            .await?;
        Ok(resp)
    }
}
