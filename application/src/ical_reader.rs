use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use domain::ical::ical_event::IcalEvent;
use url::Url;

#[async_trait]
pub trait IcalReader: Send + Sync + 'static {
    /// max: 結果の最大件数
    async fn read(
        &self,
        url: Url,
        start_time: Option<DateTime<Local>>,
        end_time: Option<DateTime<Local>>,
        max: Option<u32>,
    ) -> Result<Vec<IcalEvent>>;
}
