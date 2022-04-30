use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earthquake {
    pub url_time: Option<String>,
    pub time: String,
    pub hypocenter: String,
    pub max_seismic_intensity: String,
    pub magnitude: String,
    pub depth: String,
    pub info: String,
}
