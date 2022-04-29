use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChannels {
    pub public_channels: Vec<Channel>,
    pub dm_channels: Option<Vec<DmChannel>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub parent_id: Option<String>,
    pub archived: bool,
    pub force: bool,
    pub topic: String,
    pub name: String,
    pub children: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DmChannel {
    pub id: String,
    pub user_id: String,
}
