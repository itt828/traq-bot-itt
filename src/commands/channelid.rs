use anyhow::Result;
use traq::bot::Bot;
pub async fn channelid() -> Result<()> {
    if let Some(path) = s.next() {
        let uuid = get_channel_uuid(bot, path).await?;
        bot.post_message(channel_id, &uuid, true).await?;
    }
    Ok(())
}
