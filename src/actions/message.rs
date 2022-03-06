use crate::models::events::message::*;
use crate::patterns::is_gacha;
use crate::requests::{post_message, post_stamp};
use http::StatusCode;
use std::thread;
use std::time;
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
        if is_gacha(body.message.plain_text) {
            let content = format!(":nige_dot: https://q.trap.jp/messages/{}", body.message.id);
            thread::spawn(|| async {
                thread::sleep(time::Duration::from_secs(2));
                post_message(content, body.message.channel_id).await;
            });
        }
    }
    post_stamp(
        body.message.id,
        "e5849def-9b32-4050-a0c3-e6b73c83a822".to_string(),
        10,
    )
    .await;
    StatusCode::NO_CONTENT
}
