use chrono::Duration;
use derive_new::new;
use ulid::Ulid;
use url::Url;

#[derive(Debug, new)]
pub struct Ical {
    pub id: Ulid,
    pub owner: String,
    pub url: Url,
    pub title: Option<String>,
    pub channel: String,
    pub notification_time: Vec<Duration>,
}
