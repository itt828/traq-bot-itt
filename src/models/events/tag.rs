use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TagAdded {
    event_time: String,
    tag_id: String,
    tag: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct TagRemoved {
    event_time: String,
    tag_id: String,
    tag: String,
}

#[cfg(test)]
mod event_tag_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        tag_added_works,
        TagAdded,
        r###"
    {
  "eventTime": "2019-05-08T08:31:06.566228282Z",
  "tagId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
  "tag": "全強"
}
    "###
    );
    model_test!(
        tag_removed_works,
        TagRemoved,
        r###"
  {
  "eventTime": "2019-05-08T08:31:06.566228282Z",
  "tagId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
  "tag": "全強"
}
  "###
    );
}
