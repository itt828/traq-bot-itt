use crate::bot::Bot;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {}

impl Bot {
    pub async fn edit_message(
        &self,
        message_id: &str,
        content: &str,
        embed: bool,
    ) -> Result<Edit, Box<dyn std::error::Error>> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({ "content": content, "embed":embed });
        let resp = self
            .api_request_base(&url, Method::PUT, body)
            .await?
            .json::<Edit>()
            .await?;
        Ok(resp)
    }
}
