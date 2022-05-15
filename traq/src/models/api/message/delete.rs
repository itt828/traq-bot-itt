use crate::bot::Bot;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete {}

impl Bot {
    pub async fn delete_message(
        &self,
        message_id: &str,
    ) -> Result<Delete, Box<dyn std::error::Error>> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::DELETE, body)
            .await?
            .json::<Delete>()
            .await?;
        Ok(resp)
    }
}
