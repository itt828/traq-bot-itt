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
    pub pinned: bool,
    pub stamps: Vec<Stamp>,
    pub thread_id: Option<String>,
}
#[cfg(test)]
mod api_message_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        get_message_work,
        GetMessage,
        r###"
{
    "id": "fb30dcf8-9b34-47d0-87ea-5bc8a202a518",
    "userId": "48bf1ce8-3cf4-4470-8ac9-96a9d2fea3f4",
    "channelId": "4d902c98-15cb-446d-9b9a-06ee392c8494",
    "content": "正規表現バグらせてる",
    "createdAt": "2022-04-23T03:00:13.616884Z",
    "updatedAt": "2022-04-23T03:00:13.616884Z",
    "pinned": false,
    "stamps": [],
    "threadId": null
}    
    "###
    );
}
