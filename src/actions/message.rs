use crate::{commands, patterns::*};
use anyhow::Result;
use http::StatusCode;
use rand::prelude::*;
use std::sync::Arc;
use std::thread;
use std::time;
use traq::{bot::Bot, models::event::message::MessageCreated};
pub async fn handle_message_created(
    bot: Arc<Bot>,
    body: Arc<MessageCreated>,
) -> Result<StatusCode> {
    println!(
        "[{}] MESSAGE_CREATED from: {}, content: {}, channel_id: {}, bot: {}",
        body.event_time,
        body.message.id,
        body.message.plain_text,
        body.message.channel_id,
        body.message.user.bot,
    );
    if !body.message.user.bot {
        if is_gacha(&body.message.plain_text) {
            let content = format!(":nige_dot: https://q.trap.jp/messages/{}", body.message.id);
            let cid = body.message.channel_id.clone();
            let bt = bot.clone();
            tokio::spawn(async move {
                thread::sleep(time::Duration::from_secs(2));
                bt.post_message(&cid, &content, true).await;
            });
        }
        {
            let txt = body.message.plain_text.clone();
            let cid = body.message.channel_id.clone();
            let bt = bot.clone();
            tokio::spawn(async move {
                commands::handle_command(&bt, &txt, &cid).await;
            });
        }
    }
    if is_itt(&body.message.plain_text) {
        let r: usize = random();
        let stamps = bot.get_stamp(true).await.unwrap();

        bot.add_stamp(
            &body.message.id,
            &stamps.stamps[r % stamps.stamps.len()].id,
            100,
        )
        .await;
    }
    Ok(StatusCode::NO_CONTENT)
}
