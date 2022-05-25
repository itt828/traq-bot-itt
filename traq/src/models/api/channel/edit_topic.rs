use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde_json::json;

impl Bot {
    pub async fn edit_topic(&self, channel_id: &str, topic: &str) -> Result<()> {
        let url = format!("{}/channels/{}/topic", self.base_url, channel_id);
        let body = json!({ "topic": topic });
        let _ = self.api_request_base(&url, Method::PUT, body).await?;
        Ok(())
    }
}
