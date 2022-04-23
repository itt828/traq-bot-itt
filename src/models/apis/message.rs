use super::base::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessage {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub pinned: String,
    pub stamps: Vec<Stamp>,
    pub thread_id: String,
}
