use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamps {
    pub stamps: Vec<Stamp>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub file_id: String,
    pub is_unicode: String,
}
