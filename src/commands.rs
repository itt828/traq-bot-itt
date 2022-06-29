mod cat;
mod channelid;
mod echo;
mod shellgei;
mod subscribe;
mod unsubscribe;

use crate::patterns::extract_message_id;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use splitty::*;
use traq::{bot::Bot, utils::get_channel_uuid};

pub async fn handle_command(bot: &Bot, plain_text: &str, channel_id: &str) -> Result<()> {
    let mut s = arg_parse(plain_text);
    let mension = Regex::new(r"@(?i)bot_itt").unwrap();
    if mension.is_match(&s.0) {
        s = arg_parse(&s.1);
    }
    match s.0.as_str() {
        "subscribe" => subscribe::subscribe(&bot, channel_id).await?,
        "unsubscribe" => unsubscribe::unsubscribe(&bot, channel_id).await?,
        "echo" => echo::echo(&bot, channel_id, &s.1).await?,
        "cat" => cat::cat(&bot, channel_id, &s.1).await?,
        "channelid" => channelid::channelid(&bot, &s.1).await?,
        "shellgei" => shellgei::shellgei(&bot, &s.1).await?,

        _ => {}
    }

    Ok(())
}

fn arg_parse(s: &str) -> (String, String) {
    let words = s.clone();
    let mut words = words.split_whitespace();
    let first_word = words.next().unwrap().to_owned();
    let other_word = s.replacen(&first_word, "", 1);
    let other_word = other_word.trim_start().to_owned();
    (first_word, other_word)
}
