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
    embedded: Vec<EmbedInfo>,
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

#[cfg(test)]
mod event_base_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        message_work,
        Message,
        r###"
{
    "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
    "user": {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    },
    "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
    "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
    "plainText": "@takashi_trap こんにちは",
    "embedded": [
      {
        "raw": "@takashi_trap",
        "type": "user",
        "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
      }
    ],
    "createdAt": "2019-05-08T13:33:51.632149265Z",
    "updatedAt": "2019-05-08T13:33:51.632149265Z"
  }
    "###
    );
    model_test!(
        user_work,
        User,
        r#"
    {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    }
    "#
    );
    model_test!(
        embed_work,
        Vec<EmbedInfo>,
        r#"
  [
      {
        "raw": "@takashi_trap",
        "type": "user",
        "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
      }
    ]"#
    );
}
