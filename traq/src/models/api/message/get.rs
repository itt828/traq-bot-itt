use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub pinned: bool,
    pub stamps: Vec<Stamp>,
    pub thread_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub user_id: String,
    pub stamp_id: String,
    pub count: usize,
    pub created_at: String,
    pub updated_at: String,
}

impl Bot {
    pub async fn get_message(&self, message_id: String) -> Result<Get> {
        let url = format!("{}/messages/{}", self.base_url, message_id);
        let body = json!({});
        let resp = self
            .api_request_base(&url, Method::GET, body)
            .await?
            .json::<Get>()
            .await?;
        Ok(resp)
    }
}

crate::model_test!(
    get_test,
    Get,
    r#"{
  "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "userId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "channelId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "content": "string",
  "createdAt": "2022-05-25T05:23:20.892Z",
  "updatedAt": "2022-05-25T05:23:20.892Z",
  "pinned": true,
  "stamps": [
    {
      "userId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "stampId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "count": 0,
      "createdAt": "2022-05-25T05:23:20.892Z",
      "updatedAt": "2022-05-25T05:23:20.892Z"
    }
  ],
  "threadId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
"#
);
