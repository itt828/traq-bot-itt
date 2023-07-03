use std::{str::FromStr, sync::Arc};

use chrono::{FixedOffset, Local};
use clap::Args;
use traq::{
    apis::{bot_api::let_bot_join_channel, channel_api::post_message},
    models::{PostBotActionJoinRequest, PostMessageRequest},
};
use traq_ws_bot::events::common::Message;

use crate::Resource;

#[derive(Debug, Args)]
pub struct Time;

pub async fn handle_time(
    args: Time,
    message: Message,
    resource: Arc<Resource>,
) -> anyhow::Result<()> {
    let _ = post_message(
        &resource.configuration,
        &message.channel_id,
        Some(PostMessageRequest {
            content: format!(
                "{}",
                Local::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
            ),
            embed: None,
        }),
    )
    .await;
    Ok(())
}
