use clap::{CommandFactory, Parser};
use regex;
use shlex;
use strip_ansi_escapes;
use traq_ws_bot::events::payload;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Hoge {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn command_parser(input_args: Vec<String>) {
    let parsed = Hoge::parse_from(input_args);
    println!("{:?}", parsed);
    let cmd = <Hoge as CommandFactory>::command();
    let help_msg = cmd.disable_colored_help(true).render_help();
    println!(
        "{}",
        String::from_utf8(strip_ansi_escapes::strip(&help_msg.ansi().to_string()).unwrap())
            .unwrap()
    );
}

fn is_command_prefix(first_word: String) -> bool {
    let prefix = regex::Regex::new(r"^(?i)@bot_itt").unwrap();
    prefix.is_match(&first_word)
}

pub fn exec_command(payload: payload::MessageCreated) {
    let msg = payload.message.plain_text;
    let arg_vec = shlex::split(&msg).unwrap();
    if is_command_prefix(arg_vec[0].clone()) {
        command_parser(arg_vec);
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
    }
}
