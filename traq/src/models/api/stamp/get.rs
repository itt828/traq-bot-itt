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
        let resp = self.api_request_base(&url, Method::GET, body).await?;
        let resp = resp.json::<Get>().await?;
        Ok(resp)
    }
}
#[test]
fn a() {
    let x = r###"
 [
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "name": "kSD9BvOcQDlYIOF39DbQsD",
    "creatorId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "createdAt": "2022-04-23T13:12:48.365Z",
    "updatedAt": "2022-04-23T13:12:48.365Z",
    "fileId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "isUnicode": true
  },
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "name": "kSD9BvOcQDlYIOF39DbQsD",
    "creatorId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "createdAt": "2022-04-23T13:12:48.365Z",
    "updatedAt": "2022-04-23T13:12:48.365Z",
    "fileId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "isUnicode": true
  }
]
  "###;
    let r = serde_json::from_str::<Get>(x);
    assert!(r.is_ok());
}
