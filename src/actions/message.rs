use crate::models::events::message::*;
use crate::requests::post_message;
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
    StatusCode::NO_CONTENT
}
