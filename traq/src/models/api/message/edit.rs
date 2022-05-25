use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde_json::json;

impl Bot {
    pub async fn edit_message(&self, message_id: &str, content: &str, embed: bool) -> Result<()> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({ "content": content, "embed":embed });
        let _ = self.api_request_base(&url, Method::PUT, body).await?;
        Ok(())
    }
}
