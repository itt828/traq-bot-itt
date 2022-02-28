use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: String,
    name: String,
    path: String,
    parent_id: String,
    creator: Creator,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    id: String,
    name: String,
    display_name: String,
    icon_id: String,
    bot: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    id: String,
    user: User,
    channel_id: String,
    text: String,
    plain_text: String,
    embeded: Vec<EmbedInfo>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Updater {
    id: String,
    name: String,
    display_name: String,
    icon_id: String,
    bot: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    stamp_id: String,
    user_id: String,
    count: u32,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedInfo {
    raw: String,
    #[serde(rename = "type")]
    embed_type: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: String,
    display_name: String,
    icon_id: String,
    bot: bool,
}
