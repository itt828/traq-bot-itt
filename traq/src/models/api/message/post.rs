use crate::bot::Bot;
use crate::error::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: String,
    user_id: String,
    channel_id: String,
    content: String,
    created_at: String,
    updated_at: String,
    pinned: bool,
    stamps: Vec<Stamp>,
    thread_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    user_id: String,
    stamp_id: String,
    count: usize,
    created_at: String,
    updated_at: String,
}

impl Bot {
    pub async fn post_message(&self, channel_id: &str, content: &str, embed: bool) -> Result<Post> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        let body = json!({ "content": content, "embed":embed });
        let resp = self.api_request_base(&url, Method::POST, body).await?;
        let resp = resp.json::<Post>().await?;
        Ok(resp)
    }
}

#[test]
fn post_test() {
    let t = r##"
{
  "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "userId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "channelId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "content": "string",
  "createdAt": "2022-05-25T05:01:58.403Z",
  "updatedAt": "2022-05-25T05:01:58.403Z",
  "pinned": true,
  "stamps": [
    {
      "userId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "stampId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "count": 0,
      "createdAt": "2022-05-25T05:01:58.403Z",
      "updatedAt": "2022-05-25T05:01:58.403Z"
    }
  ],
  "threadId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
"##;
    let r = serde_json::from_str::<Post>(t);
    assert!(r.is_ok());
}
