use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChannels {
    pub public: Vec<Channel>,
    pub dm: Option<Vec<DmChannel>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub parent_id: Option<String>,
    pub archived: bool,
    pub force: bool,
    pub topic: String,
    pub name: String,
    pub children: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DmChannel {
    pub id: String,
    pub user_id: String,
}
#[cfg(test)]
mod api_channel_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        get_channels_work,
        GetChannels,
        r###"
  {
  "public": [
    {
      "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "parentId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "archived": true,
      "force": true,
      "topic": "string",
      "name": "xIIb0Jsz2krn_7hatO",
      "children": [
        "3fa85f64-5717-4562-b3fc-2c963f66afa6"
      ]
    }
  ],
  "dm": [
    {
      "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
      "userId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
    }
  ]
}
    "###
    );
}
