use anyhow::Result;
use traq::bot::Bot;

pub async fn echo(bot: &Bot, channel_id: &str, s: &str) -> Result<()> {
    for msg in s.by_ref() {
        bot.post_message(channel_id, msg, true).await?;
    }
    Ok(())
}
