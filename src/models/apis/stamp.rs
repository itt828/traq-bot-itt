use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stamp {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub file_id: String,
    pub is_unicode: bool,
}
#[cfg(test)]
mod api_stamp_tests {
    use super::*;
    use crate::model_test;

    model_test!(
        get_stamps_work,
        Vec<Stamp>,
        r###"
 [
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "name": "kSD9BvOcQDlYIOF39DbQsD",
    "creatorId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "createdAt": "2022-04-23T13:12:48.365Z",
    "updatedAt": "2022-04-23T13:12:48.365Z",
    "fileId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "isUnicode": true
  },
  {
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "name": "kSD9BvOcQDlYIOF39DbQsD",
    "creatorId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "createdAt": "2022-04-23T13:12:48.365Z",
    "updatedAt": "2022-04-23T13:12:48.365Z",
    "fileId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "isUnicode": true
  }
]
    "###
    );
}
