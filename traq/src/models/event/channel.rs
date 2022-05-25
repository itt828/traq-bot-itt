use super::base::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelCreated {
    event_time: String,
    channel: Channel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelTopicChanged {
    event_time: String,
    channel: Channel,
    topic: String,
    updater: Updater,
}

#[cfg(test)]
mod event_channel_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        channel_created_works,
        ChannelCreated,
        r###"
    {
  "eventTime": "2019-05-08T13:45:51.506206852Z",
  "channel": {
    "id": "711afb4c-23e7-46dc-b845-5160f7088ce9",
    "name": "yamada",
    "path": "#gps/yamada",
    "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
    "creator": {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    },
    "createdAt": "2019-05-08T13:45:51.487718Z",
    "updatedAt": "2019-05-08T13:45:51.487718Z"
  }
}
    "###
    );
    model_test!(
        channel_topic_changed_works,
        ChannelTopicChanged,
        r###"
      {
  "eventTime": "2019-05-09T11:32:49.505357701Z",
  "channel": {
    "id": "9aba50da-f605-4cd0-a428-5e4558cb911e",
    "name": "bot",
    "path": "#a/bot",
    "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
    "creator": {
      "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
      "name": "takashi_trap",
      "displayName": "寺田 健二",
      "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
      "bot": false
    },
    "createdAt": "2019-04-02T06:31:16.229419Z",
    "updatedAt": "2019-05-09T11:32:49.475127Z"
  },
  "topic": "トピック",
  "updater": {
    "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
    "name": "takashi_trap",
    "displayName": "寺田 健二",
    "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
    "bot": false
  }
}
      "###
    );
}
