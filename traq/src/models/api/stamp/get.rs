use crate::bot::Bot;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Get {
    stamps: Vec<Stamp>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    id: String,
    name: String,
    creator_id: String,
    created_at: String,
    updated_at: String,
    file_id: String,
    is_unicode: bool,
}

impl Bot {
    pub async fn get_stamp(
        &self,
        include_unicode: bool,
    ) -> Result<Get, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/stamps?include-unicode={}",
            self.base_url, include_unicode
        );
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::GET, body)
            .await?
            .json::<Get>()
            .await?;
        Ok(resp)
    }
}
