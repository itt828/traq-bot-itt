use crate::patterns::extract_message_id;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use splitty::*;
use traq::{bot::Bot, utils::get_channel_uuid};
pub async fn handle_command(bot: &Bot, raw_s: &str, channel_id: &str) -> Result<()> {
    let mut s = split_unquoted_char(raw_s, ' ').unwrap_quotes(true);
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
                Some("shellgei") => {
                    let re = Regex::new(r"@(?i)bot_itt\sshellgei").unwrap();

                    let req_str = re.replace(raw_s, "");
                    let client = reqwest::Client::new();
                    let body = serde_json::json!({
                      "code": req_str,
                      "images": [],
                    });
                    let resp = client
                        .post("https://websh.jiro4989.com/api/shellgei")
                        .json(&body)
                        .send()
                        .await?;
                    let resp: Value = serde_json::from_str(&resp.text().await?)?;
                    let stdout = resp["stdout"].as_str().unwrap();
                    let stderr = resp["stderr"].as_str().unwrap();
                    //                    let imgs = resp["images"]
                    //                        .as_array()
                    //                        .unwrap()
                    //                        .iter()
                    //                        .map(|x| async {
                    //                            let s = x.as_str().unwrap();
                    //                            let raw_image = base64::decode(s).unwrap();
                    //                            let image_id = bot.upload(raw_image, channel_id).await.unwrap().id;
                    //                            format!("https://q.trap.jp/files/{}", image_id)
                    //                        })
                    //                        .collect::<Vec<_>>();

                    let img = resp["images"].as_array().unwrap()[0]["image"]
                        .as_str()
                        .unwrap();
                    let raw_image = base64::decode(img).unwrap();
                    let image_id;
                    image_id = bot.upload(raw_image, channel_id).await.unwrap().id;
                    let image_url = format!("https://q.trap.jp/files/{}", image_id);

                    let msg = if stdout != "" && stderr == "" {
                        format!("{}\n{}", stdout, image_url)
                    } else {
                        format!(
                            "### stdout\n{}\n### stderr\n{}\n{}",
                            stdout, stderr, image_url
                        )
                    };
                    bot.post_message(channel_id, &msg, false).await?;
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
