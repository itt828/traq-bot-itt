use super::base::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StampCreated {
    event_time: String,
    id: String,
    name: String,
    file_id: String,
    creator: Creator,
}

#[cfg(test)]
mod event_system_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        stamp_created_works,
        StampCreated,
        r###"
  {
  "eventTime": "2019-05-08T08:31:06.566228282Z",
  "id": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
  "name": "naruhodo",
  "fileId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
  "creator": {
    "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
    "name": "takashi_trap",
    "displayName": "",
    "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
    "bot": false
  }
}
  "###
    );
}
