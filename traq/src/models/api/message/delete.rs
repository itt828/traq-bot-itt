use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde_json::json;

impl Bot {
    pub async fn delete_message(&self, message_id: &str) -> Result<()> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({});
        let _ = self.api_request_base(&url, Method::DELETE, body).await?;
        Ok(())
    }
}
