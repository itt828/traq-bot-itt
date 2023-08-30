use std::{str::FromStr, sync::Arc};

use clap::Args;
use regex::Regex;
use traq::{
    apis::{bot_api::let_bot_join_channel, channel_api::post_message},
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
    let _ = post_message(
        &resource.configuration,
        &message.channel_id,
        Some(PostMessageRequest {
            content: extract_file_id(message.text)?,
            embed: None,
        }),
    )
    .await;
    let _ = let_bot_join_channel(
        &resource.configuration,
        &resource.bot_id,
        Some(PostBotActionJoinRequest {
            channel_id: uuid::Uuid::from_str(&message.channel_id)?,
        }),
    )
    .await;
    Ok(())
}

pub fn extract_file_id(text: String) -> anyhow::Result<String> {
    let regex = Regex::new("https://q.trap.jp/files/(?<id>([0-9a-f]{8})-([0-9a-f]{4})-([0-9a-f]{4})-([0-9a-f]{4})-([0-9a-f]{12}))")?;
    let caps = regex.captures(&text);
    println!("{:?}", caps);
    Ok(caps.unwrap()["id"].to_string())
}
