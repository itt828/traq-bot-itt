use std::{
    ffi::OsString,
    io::{Read, Write},
    os::unix::prelude::OsStringExt,
    process::{Command, Stdio},
    str::FromStr,
    sync::Arc,
};

use clap::Args;
use regex::Regex;
use reqwest::{Client, Method};
use traq::{
    apis::{
        bot_api::let_bot_join_channel, channel_api::post_message, configuration::Configuration,
    },
    models::{PostBotActionJoinRequest, PostMessageRequest},
};
use traq_ws_bot::events::common::Message;

use crate::Resource;

#[derive(Debug, Args)]
pub struct PokeRecipe {
    #[arg(allow_hyphen_values = true)]
    words: Vec<String>,
}

pub async fn handle_poke_recipe(
    args: PokeRecipe,
    message: Message,
    resource: Arc<Resource>,
) -> anyhow::Result<()> {
    let file_id = extract_file_id(message.text)?;
    let image = get_traq_image_binary(&resource.configuration, file_id)
        .await
        .expect("hoge");

    let ocr_text: String = {
        let process = Command::new("tesseract")
            .arg("-")
            .arg("stdout")
            .arg("-l")
            .arg("jpn")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        process.stdin.unwrap().write_all(&image)?;
        let mut s = String::new();
        process.stdout.unwrap().read_to_string(&mut s)?;
        s
    };
    post_message(
        &resource.configuration,
        &message.channel_id,
        Some(PostMessageRequest {
            content: ocr_text,
            embed: None,
        }),
    )
    .await?;
    Ok(())
}

pub fn extract_file_id(text: String) -> anyhow::Result<String> {
    let regex = Regex::new("https://q.trap.jp/files/(?<id>([0-9a-f]{8})-([0-9a-f]{4})-([0-9a-f]{4})-([0-9a-f]{4})-([0-9a-f]{12}))")?;
    let caps = regex.captures(&text);
    Ok(caps.unwrap()["id"].to_string())
}

pub async fn get_traq_image_binary(config: &Configuration, id: String) -> anyhow::Result<Vec<u8>> {
    let auth_value = format!("Bearer {}", config.bearer_access_token.clone().unwrap());
    let client = Client::new();
    let req = client
        .request(
            Method::GET,
            format!("https://q.trap.jp/api/v3/files/{}", id),
        )
        .header("Authorization", auth_value);
    let bin = req.send().await?.bytes().await.expect("oi").to_vec();
    Ok(bin)
}

// pub fn
