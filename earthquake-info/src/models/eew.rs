use serde::{de::Unexpected, Deserialize, Deserializer, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eew {
    pub result: EewResult,
    pub report_time: String,
    pub region_code: String,
    pub request_time: String,
    pub region_name: String,
    pub longitude: String,
    #[serde(deserialize_with = "empty_string_to_none")]
    pub is_cancel: Option<bool>,
    pub depth: String,
    pub calcintensity: String,
    #[serde(deserialize_with = "empty_string_to_none")]
    pub is_final: Option<bool>,
    #[serde(deserialize_with = "empty_string_to_none")]
    pub is_training: Option<bool>,
    pub latitude: String,
    pub origin_time: String,
    pub security: EewSecurity,
    pub magunitude: String,
    pub report_num: String,
    pub request_hypo_type: String,
    pub report_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EewResult {
    pub status: String,
    pub message: String,
    pub is_auth: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EewSecurity {
    pub realm: String,
    pub hash: String,
}

struct DeserializeBoolOrEmptyStringVisitor;
impl<'de> serde::de::Visitor<'de> for DeserializeBoolOrEmptyStringVisitor {
    type Value = Option<bool>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean or a string")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Some(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "" {
            Ok(None)
        } else {
            Err(serde::de::Error::invalid_value(
                Unexpected::Str(&v),
                &"invalid region name",
            ))
        }
    }
}
fn empty_string_to_none<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeBoolOrEmptyStringVisitor)
}
