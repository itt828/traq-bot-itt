use crate::apis::message::{post_message, post_stamp};
use crate::apis::stamp::get_stamps;
use crate::commands;
use crate::models::events::message::*;
use crate::patterns::is_gacha;
use http::StatusCode;
use rand::prelude::*;
use std::sync::Arc;
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
    let txt = std::sync::Arc::new(body.message.plain_text);
    let cid = std::sync::Arc::new(body.message.channel_id);
    if !body.message.user.bot {
        if is_gacha(&txt) {
            let content = Arc::new(format!(
                ":nige_dot: https://q.trap.jp/messages/{}",
                body.message.id
            ));
            let cid = cid.clone();
            tokio::spawn(async move {
                thread::sleep(time::Duration::from_secs(2));
                post_message(&content.clone(), &cid).await;
            });
        }
        {
            let txt = txt.clone();
            let cid = cid.clone();
            tokio::spawn(async move {
                commands::handle_command(&txt, &cid).await;
            });
        }
    }
    {
        let r: usize = random();
        let stamps = get_stamps().await;

        post_stamp(
            &body.message.id,
            &stamps.stamps[r % stamps.stamps.len()].id,
            100,
        )
        .await;
    }
    StatusCode::NO_CONTENT
}
