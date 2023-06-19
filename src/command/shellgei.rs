use std::sync::Arc;

use clap::Args;
use regex::Regex;
use traq::{apis::channel_api::post_message, models::PostMessageRequest};
use traq_ws_bot::events::common::Message;

use crate::Resource;

const SHELLGEI_PREFIX: &str = concat!(r"^(?i)(@bot_itt|cmd)", r"\s+(shellgei|sg)\n*");

#[derive(Debug, Args)]
pub struct Shellgei {
    #[arg(allow_hyphen_values = true)]
    words: Vec<String>,
}

pub async fn handle_shellgei(args: Shellgei, message: Message, resource: Arc<Resource>) {
    let regex = Regex::new(SHELLGEI_PREFIX).unwrap();
    let payload = regex.replace(&message.plain_text, "");

    let resp = post_message(
        &resource.configuration,
        &message.channel_id,
        Some(PostMessageRequest {
            content: payload.into(),
            embed: None,
        }),
    )
    .await;
}
