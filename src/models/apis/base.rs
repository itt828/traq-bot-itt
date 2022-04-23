use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub user_id: String,
    pub stamp_id: String,
    pub count: u64,
    pub created_at: String,
    pub updated_at: String,
}
