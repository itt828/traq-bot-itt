use anyhow::Result;
use traq::bot::Bot;

pub async fn cat(bot: &Bot, channel_id: &str, s: &str) -> Result<()> {
    for msg in s.by_ref() {
        let mid = extract_message_id(msg).unwrap();
        let got_msg = bot.get_message(mid).await?;
        bot.post_message(channel_id, &got_msg.content, true).await?;
    }
    Ok(())
}
