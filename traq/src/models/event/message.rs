use super::base::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageCreated {
    pub event_time: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeleted {
    pub event_time: String,
    pub message: DelMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelMessage {
    pub id: String,
    pub channel_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageUpdated {
    pub event_time: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageCreated {
    pub event_time: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageDeleted {
    pub event_time: String,
    pub message: DelMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageUpdated {
    pub event_time: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMessageStampsUpdated {
    pub event_time: String,
    pub message_id: String,
    pub stamps: Vec<Stamp>,
}

#[cfg(test)]
mod event_message_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        message_created_work,
        MessageCreated,
        r###"
{
  "eventTime": "2019-05-08T13:33:51.690308239Z",
  "message": {
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
}
    "###
    );
    model_test!(
        message_deleted,
        MessageDeleted,
        r###"
  {
  "eventTime": "2019-05-08T13:33:51.690308239Z",
  "message": {
    "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
    "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e"
  }
}
  "###
    );
    model_test!(
        message_updated,
        MessageUpdated,
        r###"
    {
  "eventTime": "2019-05-08T13:33:51.690308239Z",
  "message": {
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
}
    "###
    );
    model_test!(
        direct_message_created_works,
        DirectMessageCreated,
        r###"
{
  "eventTime": "2019-05-08T13:36:09.421492525Z",
  "message": {
    "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
    "user": {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    },
    "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34",
    "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
    "plainText": "@takashi_trap こんにちは",
    "embedded": [
      {
        "raw": "@takashi_trap",
        "type": "user",
        "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
      }
    ],
    "createdAt": "2019-05-08T13:36:09.365393261Z",
    "updatedAt": "2019-05-08T13:36:09.365393261Z"
  }
}
"###
    );
    model_test!(
        direct_message_deleted_works,
        DirectMessageDeleted,
        r###"
    {
  "eventTime": "2019-05-08T13:36:09.421492525Z",
  "message": {
    "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
    "userId": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
    "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34"
  }
}
    "###
    );
    model_test!(
        direct_message_updated_works,
        DirectMessageUpdated,
        r###"
{
  "eventTime": "2019-05-08T13:36:09.421492525Z",
  "message": {
    "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
    "user": {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    },
    "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34",
    "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
    "plainText": "@takashi_trap こんにちは",
    "embedded": [
      {
        "raw": "@takashi_trap",
        "type": "user",
        "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
      }
    ],
    "createdAt": "2019-05-08T13:36:09.365393261Z",
    "updatedAt": "2019-05-08T13:36:09.365393261Z"
  }
}
    "###
    );
    model_test!(
        bot_message_stamps_updated_works,
        BotMessageStampsUpdated,
        r###"
    {
  "eventTime": "2020-10-17T03:35:34.5326265Z",
  "messageId": "200b6600-b2cd-4c1e-b366-9c40308cc087",
  "stamps": [
    {
      "stampId": "1cd58034-8998-4b1c-afe4-fcd591354a97",
      "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
      "count": 22,
      "createdAt": "2020-10-17T03:35:17.89545Z",
      "updatedAt": "2020-10-17T03:35:34Z"
    },
    {
      "stampId": "6fc62b49-dea0-45b8-8c0c-38035082b111",
      "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
      "count": 23,
      "createdAt": "2020-10-17T03:35:17.737127Z",
      "updatedAt": "2020-10-17T03:35:34Z"
    },
    {
      "stampId": "b77fad4e-b63f-42a2-916c-5cfe5af3d8b9",
      "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
      "count": 24,
      "createdAt": "2020-10-17T03:34:56.575099Z",
      "updatedAt": "2020-10-17T03:35:34Z"
    }
  ]
}
    "###
    );
}
