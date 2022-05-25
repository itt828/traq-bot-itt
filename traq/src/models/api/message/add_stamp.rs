use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddStamp {}

impl Bot {
    pub async fn add_stamp(
        &self,
        message_id: &str,
        stamp_id: &str,
        count: usize,
    ) -> Result<AddStamp> {
        let url = format!(
            "{}/messages/{}/stamps/{}",
            self.base_url, message_id, stamp_id
        );
        let body = json!({ "count": count });
        let resp = self
            .api_request_base(&url, Method::POST, body)
            .await?
            .json::<AddStamp>()
            .await?;
        Ok(resp)
    }
}
