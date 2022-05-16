use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditSubscription {}

impl Bot {
    pub async fn edit_subscription(
        &self,
        channel_id: &str,
        on_users: &Vec<String>,
        off_users: &Vec<String>,
    ) -> Result<EditSubscription> {
        let url = format!("{}/channels/{}/subscribers", self.base_url, channel_id);
        let body = json!({ "on": on_users,"off":off_users });
        let resp = self
            .api_request_base(&url, Method::PATCH, body)
            .await?
            .json::<EditSubscription>()
            .await?;
        Ok(resp)
    }
}
