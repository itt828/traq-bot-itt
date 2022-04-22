use crate::{
    models::apis::message::{get_message, post_message},
    patterns::extract_message_id,
};
use regex::Regex;
use splitty::*;

pub async fn handle_command(s: &String, channel_id: &String) {
    let mut s = split_unquoted_char(s.as_str(), ' ').unwrap_quotes(true);
    if let Some(mention) = s.next() {
        let re = Regex::new(r"@(?i)bot_itt").unwrap();
        if re.is_match(mention) {
            match s.next() {
                Some("cat") => {
                    while let Some(msg) = s.next() {
                        post_message(&String::from(msg), &channel_id).await;
                    }
                }
                Some("echo") => {
                    while let Some(msg) = s.next() {
                        let mid = extract_message_id(msg).unwrap();
                        let got_msg = get_message(&mid).await;
                        post_message(&got_msg, &channel_id).await;
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
}
