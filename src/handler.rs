use crate::Resource;
use std::sync::Arc;
use traq::{apis::message_api::post_message, models::PostMessageRequest};
use traq_ws_bot::events::payload;
pub async fn message_created_handler(payload: payload::MessageCreated, resource: Arc<Resource>) {
    let msg = payload.message.text;
    let _ = post_message(
        &resource.configuration,
        &payload.message.channel_id,
        Some(PostMessageRequest {
            content: msg.clone(),
            embed: Some(true),
        }),
    )
    .await;
}

pub async fn direct_message_created_handler(
    payload: payload::DirectMessageCreated,
    resource: Arc<Resource>,
) {
    let msg = payload.message.text;
    let _ = post_message(
        &resource.configuration,
        &payload.message.channel_id,
        Some(PostMessageRequest {
            content: msg.clone(),
            embed: Some(true),
        }),
    )
    .await;
}
