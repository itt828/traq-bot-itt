mod handler;
use crate::handler::{direct_message_created_handler, message_created_handler};
use traq::apis::configuration::Configuration;
use traq_ws_bot::bot::builder;

pub struct Resource {
    configuration: Configuration,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").expect("BOT_ACCESS_TOKEN not set");
    let config = Configuration {
        bearer_access_token: Some(bot_access_token.clone()),
        ..Default::default()
    };
    let resource = Resource {
        configuration: config,
    };

    let bot = builder(bot_access_token)
        .insert_resource(resource)
        .on_message_created_with_resource(message_created_handler)
        .on_direct_message_created_with_resource(direct_message_created_handler)
        .build();
    bot.start().await
}
