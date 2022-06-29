use anyhow::Result;
use traq::bot::Bot;

pub async fn subscribe(bot: &Bot, channel_id: &str) -> Result<()> {
    let bot_id = std::env::var("BOT_ID")?;
    bot.join(&bot_id, channel_id).await?;
    bot.post_message(channel_id, "おいすー", true).await?;
    Ok(())
}
