use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetSubscription {
    user_ids: Vec<String>,
}

impl Bot {
    pub async fn get_subscription(&self, channel_id: &str) -> Result<GetSubscription> {
        let url = format!("{}/channels/{}/subscribers", self.base_url, channel_id);
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::GET, body)
            .await?
            .json::<GetSubscription>()
            .await?;
        Ok(resp)
    }
}

crate::model_test!(
    get_subscription_test,
    GetSubscription,
    r#"
[
  "3fa85f64-5717-4562-b3fc-2c963f66afa6"
]
"#
);
