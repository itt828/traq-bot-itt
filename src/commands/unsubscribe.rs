use anyhow::Result;
use traq::bot::Bot;
pub async fn unsubscribe(bot: &Bot, channel_id: &str) -> Result<()> {
    bot.leave("", channel_id).await?;
    bot.post_message(channel_id, "ーすいお", true).await?;
    Ok(())
}
