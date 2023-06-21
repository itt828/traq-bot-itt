use std::{str::FromStr, sync::Arc};

use clap::Args;
use traq::{
    apis::{bot_api::let_bot_leave_channel, channel_api::post_message},
    models::{PostBotActionLeaveRequest, PostMessageRequest},
};
use traq_ws_bot::events::common::Message;

use crate::Resource;

#[derive(Debug, Args)]
pub struct Leave {
    channel_id: Option<String>,
}

pub async fn handle_leave(
    args: Leave,
    message: Message,
    resource: Arc<Resource>,
) -> anyhow::Result<()> {
    let channel_id = match args.channel_id {
        Some(s) => s,
        None => message.channel_id,
    };
    let _ = post_message(
        &resource.configuration,
        &channel_id,
        Some(PostMessageRequest {
            content: format!(":wave:"),
            embed: None,
        }),
    )
    .await;
    let _ = let_bot_leave_channel(
        &resource.configuration,
        &resource.bot_id,
        Some(PostBotActionLeaveRequest {
            channel_id: uuid::Uuid::from_str(&channel_id)?,
        }),
    )
    .await;
    Ok(())
}
