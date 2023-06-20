use std::{str::FromStr, sync::Arc};

use clap::Args;
use traq::{
    apis::{bot_api::let_bot_join_channel, channel_api::post_message},
    models::{PostBotActionJoinRequest, PostMessageRequest},
};
use traq_ws_bot::events::common::Message;

use crate::Resource;

#[derive(Debug, Args)]
pub struct Join {
    channel_id: Option<String>,
}

pub async fn handle_join(
    args: Join,
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
            content: format!("おいすー"),
            embed: None,
        }),
    )
    .await;
    let _ = let_bot_join_channel(
        &resource.configuration,
        &resource.bot_id,
        Some(PostBotActionJoinRequest {
            channel_id: uuid::Uuid::from_str(&channel_id)?,
        }),
    )
    .await;
    Ok(())
}
