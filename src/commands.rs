use crate::patterns::extract_message_id;
use anyhow::Result;
use regex::Regex;
use splitty::*;
use traq::{bot::Bot, utils::get_channel_uuid};
pub async fn handle_command(bot: &Bot, s: &str, channel_id: &str) -> Result<()> {
    let mut s = split_unquoted_char(s, ' ').unwrap_quotes(true);
    if let Some(mention) = s.next() {
        let re = Regex::new(r"@(?i)bot_itt").unwrap();
        if re.is_match(mention) {
            match s.next() {
                Some("subscribe") => {
                    bot.join("", channel_id).await?;
                    bot.post_message(channel_id, "おいすー", true).await?;
                }
                Some("unsubscribe") => {
                    bot.leave("", channel_id).await?;
                    bot.post_message(channel_id, "ーすいお", true).await?;
                }
                Some("echo") => {
                    for msg in s.by_ref() {
                        bot.post_message(channel_id, msg, true).await?;
                    }
                }
                Some("cat") => {
                    for msg in s.by_ref() {
                        let mid = extract_message_id(msg).unwrap();
                        let got_msg = bot.get_message(mid).await?;
                        bot.post_message(channel_id, &got_msg.content, true).await?;
                    }
                }
                Some("channelid") => {
                    if let Some(path) = s.next() {
                        let uuid = get_channel_uuid(bot, path).await?;
                        bot.post_message(channel_id, &uuid, true).await?;
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
