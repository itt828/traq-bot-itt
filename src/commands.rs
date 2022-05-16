use crate::patterns::extract_message_id;
use anyhow::Result;
use regex::Regex;
use splitty::*;
use traq::{bot::Bot, utils::get_channel_uuid};
pub async fn handle_command(bot: &Bot, s: &String, channel_id: &String) -> Result<()> {
    let mut s = split_unquoted_char(s.as_str(), ' ').unwrap_quotes(true);
    if let Some(mention) = s.next() {
        let re = Regex::new(r"@(?i)bot_itt").unwrap();
        if re.is_match(mention) {
            match s.next() {
                Some("subscribe") => {
                    bot.join("", channel_id).await;
                    bot.post_message(&String::from("おいすー"), channel_id, true)
                        .await;
                }
                Some("unsubscribe") => {
                    bot.leave("", channel_id).await;
                    bot.post_message(&String::from("ーすいお"), channel_id, true)
                        .await;
                }
                Some("echo") => {
                    while let Some(msg) = s.next() {
                        bot.post_message(&String::from(msg), channel_id, true).await;
                    }
                }
                Some("cat") => {
                    while let Some(msg) = s.next() {
                        let mid = extract_message_id(msg).unwrap();
                        let got_msg = bot.get_message(mid).await?;
                        bot.post_message(channel_id, &got_msg.content, true).await;
                    }
                }
                Some("channelid") => {
                    if let Some(path) = s.next() {
                        let uuid = get_channel_uuid(bot, path).await?;
                        bot.post_message(&uuid, channel_id, true).await;
                    }
                }
                Some("count") => {
                    match s.next() {
                        Some("group") => {
                            println!("gr");
                        }
                        Some("member") => {
                            println!("me");
                        }
                        Some(_) => {
                            println!("cnt_");
                        }
                        _ => {
                            println!("cnt");
                        }
                    }
                    println!("{:?}", s.collect::<Vec<_>>());
                }
                Some("help") => {
                    println!("help");
                }
                Some(_) => {
                    println!("help_");
                }
                _ => {
                    println!("_");
                }
            }
        }
    }
    Ok(())
}
