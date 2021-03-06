use crate::bot::Bot;
use crate::error::*;
use reqwest::{multipart::Part, Client, Method};
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
    #[serde(rename = "type")]
    pub thumbnail_type: String,
    pub mime: String,
    pub width: usize,
    pub height: usize,
}

impl Bot {
    pub async fn upload(
        &self,
        file: Vec<u8>,
        mime_type: &str,
        file_name: &str,
        channel_id: &str,
    ) -> Result<Upload> {
        let auth_value = format!("Bearer {}", self.bot_access_token);

        let channel_id = channel_id.to_string();

        let url = format!("{}/files", self.base_url);
        let client = Client::new();
        let file = Part::bytes(file)
            .file_name(file_name.to_string())
            .mime_str(mime_type)?;
        let form = reqwest::multipart::Form::new()
            .text("channelId", channel_id)
            .part("file", file);
        let req = client
            .request(Method::POST, url)
            .header("Authorization", auth_value)
            .multipart(form);
        println!("{:?}", req);

        let resp = req.send().await?;
        println!("{:?}", resp);
        let resp = resp.json::<Upload>().await?;
        println!("{:?}", resp);
        Ok(resp)
    }
}
