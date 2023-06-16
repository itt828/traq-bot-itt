use clap::Args;
use traq::apis::{bot_api::let_bot_join_channel, configuration::Configuration};

#[derive(Debug, Args)]
pub struct Join {
    ChannelId: Option<String>,
}

pub fn exec_join(config: &Configuration, bot_id: &str) {
    let _ = let_bot_join_channel(config, bot_id, None);
}
