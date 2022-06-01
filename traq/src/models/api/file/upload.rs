use crate::bot::Bot;
use crate::error::*;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: String,
    pub name: String,
    pub mime: String,
    pub size: usize,
    pub md5: String,
    pub is_animated_image: bool,
    pub created_at: String,
    pub thumbnails: Vec<ThumbnailInfo>,
    pub channel_id: String,
    pub uploader_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailInfo {
    pub thumbnail_type: String,
    pub mime: String,
    pub width: usize,
    pub height: usize,
}

impl Bot {
    pub async fn upload(&self, file: &str, channel_id: &str) -> Result<Upload> {
        let auth_value = format!("Bearer {}", self.bot_access_token);

        let file = file.to_string();
        let channel_id = channel_id.to_string();

        let url = format!("{}/files", self.base_url);
        let client = Client::new();
        let form = reqwest::multipart::Form::new()
            .text("file", file)
            .text("channelId", channel_id);
        let resp = client
            .request(Method::POST, url)
            .header("Authorization", auth_value)
            .header("Content-Type", "multipart/form-data")
            .multipart(form)
            .send()
            .await?;
        println!("{:?}", resp);
        let resp = resp.json::<Upload>().await?;
        println!("{:?}", resp);
        Ok(resp)
    }
}
