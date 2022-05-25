use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Get {
    pub stamps: Vec<Stamp>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub file_id: String,
    pub is_unicode: bool,
}

impl Bot {
    pub async fn get_stamp(&self, include_unicode: bool) -> Result<Get> {
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
