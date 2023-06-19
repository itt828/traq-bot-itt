mod command;
mod cron;
mod handler;
use std::sync::Arc;

use crate::handler::{direct_message_created_handler, message_created_handler};
use cron::cron_jobs;
use traq::apis::configuration::Configuration;
use traq_ws_bot::bot::builder;

const GPS_EARTHQUAKE: &str = "0043558c-6efb-4a01-8a21-fcb171190f64";
const COMMAND_PREFIX: &str = r"^(?i)(@bot_itt|cmd)";

pub struct Resource {
    configuration: Arc<Configuration>,
    bot_id: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not set");
    let config = Configuration {
        bearer_access_token: Some(bot_access_token.clone()),
        ..Default::default()
    };
    let resource = Resource {
        configuration: Arc::new(config),
        bot_id: String::from("78d4d314-7b1d-4486-bbbf-65759fe77033"),
    };
    let config_clone = resource.configuration.clone();
    tokio::spawn(async move {
        let _ = cron_jobs(config_clone).await;
    });

    let bot = builder(bot_access_token)
        .insert_resource(resource)
        .on_message_created_with_resource(message_created_handler)
        .on_direct_message_created_with_resource(direct_message_created_handler)
        .build();
    bot.start().await
}
