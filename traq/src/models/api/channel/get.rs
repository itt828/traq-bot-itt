use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get {
    pub public: Vec<Channel>,
    pub dm: Option<Vec<DmChannel>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub parent_id: Option<String>,
    pub archived: bool,
    pub force: bool,
    pub topic: String,
    pub name: String,
    pub children: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DmChannel {
    pub id: String,
    pub user_id: String,
}

impl Bot {
    pub async fn get_channels(&self) -> Result<Get> {
        let url = format!("{}/channels/", self.base_url);
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::GET, body)
            .await?
            .json::<Get>()
            .await?;
        Ok(resp)
    }
}
