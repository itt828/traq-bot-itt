use crate::bot::Bot;
pub async fn get_channel_uuid(bot: &Bot, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut cur_parent: Option<String> = None;
    let channels = bot.get_channels().await?;
    for node in path.split('/') {
        let node = node.replace("#", "");
        let p = channels
            .public
            .iter()
            .find(|x| x.name == node && x.parent_id == cur_parent)
            .unwrap()
            .id
            .clone();
        cur_parent = Some(p);
    }
    Ok(cur_parent.unwrap())
}
