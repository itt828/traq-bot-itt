use std::sync::Arc;

use base64::{engine::general_purpose, Engine};
use clap::Args;
use regex::Regex;
use serde::Deserialize;
use traq::{apis::channel_api::post_message, models::PostMessageRequest};
use traq_ws_bot::events::common::Message;

use crate::{utils::traq_file_upload, Resource};

const SHELLGEI_PREFIX: &str = concat!(r"^(?i)(@bot_itt|cmd)", r"\s+(shellgei|sg)\n*");

#[derive(Debug, Args)]
pub struct Shellgei {
    #[arg(allow_hyphen_values = true)]
    words: Vec<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ShellgeiResponse {
    status: u32,
    system_message: String,
    stdout: String,
    stderr: String,
    images: Vec<ShellgeiImage>,
    elapsed_time: String,
}
#[derive(Deserialize)]
pub struct ShellgeiImage {
    image: String,
    format: String,
}

pub async fn handle_shellgei(
    _args: Shellgei,
    message: Message,
    resource: Arc<Resource>,
) -> anyhow::Result<()> {
    let regex = Regex::new(SHELLGEI_PREFIX)?;
    let payload = regex.replace(&message.plain_text, "");
    let body = serde_json::json!({
        "code" : payload,
        "images": [],
    });
    let resp = reqwest::Client::new()
        .post("https://websh.jiro4989.com/api/shellgei")
        .json(&body)
        .send()
        .await?
        .json::<ShellgeiResponse>()
        .await?;
    let channel_id = Arc::new(message.channel_id);
    let image_urls = resp.images.iter().map(|f| {
        let image_decode = general_purpose::STANDARD.decode(&f.image).unwrap();
        let format = Arc::new(f.format.clone());
        let resource_clone = resource.clone();
        let channel_id = channel_id.clone();
        async move {
            traq_file_upload(
                &resource_clone.configuration,
                image_decode,
                &format!("image/{}", format),
                &format!("shellgei.{}", format),
                &channel_id,
            )
            .await
            .unwrap()
        }
    });

    let images = futures::future::join_all(image_urls).await;
    let msg = format!(
        "{}\n{}",
        resp.stdout,
        images.into_iter().collect::<Vec<_>>().join("\n")
    );
    let _ = post_message(
        &resource.clone().configuration,
        &channel_id,
        Some(PostMessageRequest {
            content: msg,
            embed: None,
        }),
    )
    .await;
    Ok(())
}
