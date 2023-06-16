mod command;
mod handler;
use crate::handler::{direct_message_created_handler, message_created_handler};
use command::{command_parser, exec_command, make_error_string, split_words, Hoge};
use traq::apis::configuration::Configuration;
use traq_ws_bot::bot::builder;

pub struct Resource {
    configuration: Configuration,
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
        configuration: config,
        bot_id: String::from("78d4d314-7b1d-4486-bbbf-65759fe77033"),
    };

    let bot = builder(bot_access_token)
        .insert_resource(resource)
        .on_message_created_with_resource(message_created_handler)
        .on_direct_message_created_with_resource(direct_message_created_handler)
        .build();
    bot.start().await
}

// async fn main() {
//     aa(command_parser(split_words("@bot_itt join --help")).await);
// }
// fn aa(a: Result<Hoge, clap::error::Error>) {
//     match a {
//         Ok(x) => println!("{:#?}", x),
//         Err(e) => {
//             let error_string = make_error_string(e);
//             println!("{}", error_string);
//         }
//     }
// }
