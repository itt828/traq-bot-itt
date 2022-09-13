use chrono::{DateTime, Local};
#[derive(Debug)]
pub struct IcalEvent {
    pub title: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub calendar_name: String,
    pub summary: String,
}
