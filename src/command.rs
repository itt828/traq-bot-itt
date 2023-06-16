use std::sync::Arc;

use clap::{Parser, Subcommand};
use regex;
use shlex;
use strip_ansi_escapes;
use traq::{apis::message_api::post_message, models::PostMessageRequest};
use traq_ws_bot::events::common::Message;

use crate::Resource;

use self::join::Join;

pub mod join;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Hoge {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Join(Join),
}

pub async fn handle_subcommands(
    subcommands: SubCommands,
    message: Message,
    resource: Arc<Resource>,
) {
    match subcommands {
        SubCommands::Join(x) => join::handle_join(x, message, resource).await,
    }
}

pub async fn command_parser(input_args: Vec<String>) -> Result<Hoge, clap::error::Error> {
    Hoge::try_parse_from(input_args)
}

fn is_command_prefix(first_word: String) -> bool {
    let prefix = regex::Regex::new(r"^(?i)(@bot_itt|cmd)").unwrap();
    prefix.is_match(&first_word)
}

pub fn split_words(msg: &str) -> Vec<String> {
    shlex::split(msg).unwrap()
}

pub fn make_error_string(error: clap::Error) -> String {
    let error_string =
        String::from_utf8(strip_ansi_escapes::strip(&error.render().to_string()).unwrap()).unwrap();
    error_string
}

pub async fn exec_command(message: Message, resource: Arc<Resource>) {
    let msg = message.plain_text.clone();
    let arg_vec = split_words(&msg);

    if is_command_prefix(arg_vec[0].clone()) {
        let parsed = command_parser(arg_vec).await;
        match parsed {
            Ok(x) => handle_subcommands(x.subcommand, message, resource).await,
            Err(e) => {
                let error_string = make_error_string(e);
                let _ = post_message(
                    &resource.configuration,
                    &message.channel_id,
                    Some(PostMessageRequest {
                        content: format!("```\n{}\n```", error_string),
                        embed: None,
                    }),
                )
                .await;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_is_command() {
        use super::*;
        assert_eq!(is_command_prefix("@BOT_itt hoge".to_string()), true);
        assert_eq!(is_command_prefix("@bot_itt hoge".to_string()), true);
        assert_eq!(is_command_prefix("@botitt hoge".to_string()), false);
        assert_eq!(is_command_prefix("bot_itt hoge".to_string()), false);
        assert_eq!(is_command_prefix("hoge".to_string()), false);
        assert_eq!(is_command_prefix("cmd".to_string()), true);
        assert_eq!(is_command_prefix("cMd".to_string()), true);
    }
}
