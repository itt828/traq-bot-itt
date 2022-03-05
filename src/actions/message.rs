use crate::models::events::message::*;
use crate::requests::{post_message, post_stamp};
use http::StatusCode;

pub async fn handle_message_created(body: MessageCreated) -> StatusCode {
    println!(
        "[{}] MESSAGE_CREATED from: {}, content: {}, channel_id: {}, bot: {}",
        body.event_time,
        body.message.id,
        body.message.plain_text,
        body.message.channel_id,
        body.message.user.bot,
    );
    if !body.message.user.bot {
        post_message(body.message.plain_text, body.message.channel_id).await;
    }
    post_stamp(
        body.message.id,
        "e5849def-9b32-4050-a0c3-e6b73c83a822".to_string(),
        10,
    )
    .await;
    StatusCode::NO_CONTENT
}
