use reqwest::{multipart::Part, Client, Method};
use serde_json::Value;
use traq::apis::configuration::Configuration;

pub async fn traq_file_upload(
    config: &Configuration,
    file: Vec<u8>,
    mime_type: &str,
    file_name: &str,
    channel_id: &str,
) -> anyhow::Result<String> {
    let auth_value = format!("Bearer {}", config.bearer_access_token.clone().unwrap());
    let channel_id = channel_id.to_string();
    let client = Client::new();
    let file = Part::bytes(file)
        .file_name(file_name.to_string())
        .mime_str(mime_type)?;
    let form = reqwest::multipart::Form::new()
        .text("channelId", channel_id)
        .part("file", file);
    let req = client
        .request(Method::POST, "https://q.trap.jp/api/v3/files")
        .header("Authorization", auth_value)
        .multipart(form);

    let resp = req.send().await;
    let resp = resp?.json::<Value>().await?;
    Ok(format!(
        "https://q.trap.jp/files/{}",
        resp["id"].as_str().unwrap()
    ))
}
