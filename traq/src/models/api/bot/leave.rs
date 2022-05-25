use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leave {
    pub channel_id: String,
}

impl Bot {
    pub async fn leave(&self, bot_id: &str, channel_id: &str) -> Result<Leave> {
        let url = format!("{}/bots/{}/actions/leave", self.base_url, bot_id);
        let body = json!({ "channelId": channel_id });
        let resp = self
            .api_request_base(&url, Method::POST, body)
            .await?
            .json::<Leave>()
            .await?;
        Ok(resp)
    }
}

crate::model_test!(
    leave_test,
    Leave,
    r#"{
  "channelId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
"#
);
