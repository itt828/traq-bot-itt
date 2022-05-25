use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde_json::json;

impl Bot {
    pub async fn add_stamp(&self, message_id: &str, stamp_id: &str, count: usize) -> Result<()> {
        let url = format!(
            "{}/messages/{}/stamps/{}",
            self.base_url, message_id, stamp_id
        );
        let body = json!({ "count": count });
        let _ = self.api_request_base(&url, Method::POST, body).await?;
        Ok(())
    }
}
