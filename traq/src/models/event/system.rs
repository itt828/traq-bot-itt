use super::base::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ping {
    pub event_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joined {
    pub event_time: String,
    pub channel: Channel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Left {
    pub event_time: String,
    pub channel: Channel,
}

#[cfg(test)]
mod event_system_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        ping_works,
        Ping,
        r#"
          {
              "eventTime": "2019-05-07T04:50:48.582586882Z"
          }"#
    );
    model_test!(
        joined_works,
        Joined,
        r##"
          {
            "eventTime": "2019-05-08T13:49:13.769110201Z",
            "channel": {
              "id": "f86c925c-3002-4ba5-939a-c92344e534f9",
              "name": "po",
              "path": "#a/po",
              "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
              "creator": {
                "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
                "name": "takashi_trap",
                "displayName": "寺田 健二",
                "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
                "bot": false
              },
              "createdAt": "2018-04-25T12:22:02Z",
              "updatedAt": "2018-04-25T12:22:02Z"
            }
          }"##
    );
    model_test!(
        left_works,
        Left,
        r##"
          {
        "eventTime": "2019-05-08T13:49:16.497848449Z",
        "channel": {
          "id": "f86c925c-3002-4ba5-939a-c92344e534f9",
          "name": "po",
          "path": "#a/po",
          "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
          "creator": {
            "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
            "name": "takashi_trap",
            "displayName": "寺田 健二",
            "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
            "bot": false
          },
          "createdAt": "2018-04-25T12:22:02Z",
          "updatedAt": "2018-04-25T12:22:02Z"
        }
      }"##
    );
}
