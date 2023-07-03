use std::sync::Arc;

use clap::{Parser, Subcommand};
use traq::{apis::message_api::post_message, models::PostMessageRequest};
use traq_ws_bot::events::common::Message;

use crate::{Resource, COMMAND_PREFIX};

use self::{join::Join, leave::Leave, shellgei::Shellgei, time::Time};

pub mod join;
pub mod leave;
pub mod shellgei;
pub mod time;

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
    Leave(Leave),
    #[command(alias = "sg")]
    Shellgei(Shellgei),
    Time(Time),
}

pub async fn handle_subcommands(
    subcommands: SubCommands,
    message: Message,
    resource: Arc<Resource>,
) -> anyhow::Result<()> {
    match subcommands {
        SubCommands::Join(args) => join::handle_join(args, message, resource).await?,
        SubCommands::Leave(args) => leave::handle_leave(args, message, resource).await?,
        SubCommands::Shellgei(args) => shellgei::handle_shellgei(args, message, resource).await?,
        SubCommands::Time(args) => time::handle_time(args, message, resource).await?,
    }
    Ok(())
}

pub async fn command_parser(input_args: &Vec<String>) -> Result<Hoge, clap::error::Error> {
    Hoge::try_parse_from(input_args)
}

fn is_command_prefix(first_word: &str) -> anyhow::Result<bool> {
    let prefix = regex::Regex::new(COMMAND_PREFIX)?;
    Ok(prefix.is_match(first_word))
}

pub fn split_words(msg: &str) -> Option<Vec<String>> {
    shlex::split(msg)
}

pub fn make_error_string(error: clap::Error) -> anyhow::Result<String> {
    let error_string = String::from_utf8(strip_ansi_escapes::strip(error.render().to_string())?)?;
    Ok(error_string)
}

pub async fn exec_command(message: Message, resource: Arc<Resource>) -> anyhow::Result<()> {
    let msg = message.plain_text.clone();
    let arg_vec = split_words(&msg);
    match arg_vec {
        Some(args) => {
            if is_command_prefix(&args[0])? {
                let parsed = command_parser(&args).await;
                match parsed {
                    Ok(x) => handle_subcommands(x.subcommand, message, resource).await?,
                    Err(e) => {
                        let error_string = make_error_string(e);
                        let _msg = post_message(
                            &resource.configuration,
                            &message.channel_id,
                            Some(PostMessageRequest {
                                content: format!("```\n{}\n```", error_string?),
                                embed: None,
                            }),
                        )
                        .await?;
                    }
                };
            }
            Ok(())
        }
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    #[test]
    fn test_is_command() -> anyhow::Result<()> {
        use super::*;
        assert!(is_command_prefix("@BOT_itt hoge")?);
        assert!(is_command_prefix("@bot_itt hoge")?);
        assert!(is_command_prefix("@botitt hoge")?);
        assert!(is_command_prefix("bot_itt hoge")?);
        assert!(is_command_prefix("hoge")?);
        assert!(is_command_prefix("cmd")?);
        assert!(is_command_prefix("cMd")?);
        Ok(())
    }
}
