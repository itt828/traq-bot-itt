use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Join {
    pub channel_id: String,
}

impl Bot {
    pub async fn join(&self, bot_id: &str, channel_id: &str) -> Result<Join> {
        let url = format!("{}/bots/{}/actions/join", self.base_url, bot_id);
        let body = json!({ "channelId": channel_id });
        let resp = self
            .api_request_base(&url, Method::POST, body)
            .await?
            .json::<Join>()
            .await?;
        Ok(resp)
    }
}
