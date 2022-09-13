use std::collections::HashMap;

use anyhow::Result;
use chrono::Duration;
use domain::ical::{ical::Ical, ical_event::IcalEvent};
use ulid::Ulid;
use url::Url;

use crate::repository::ical::UpdateArgs;

pub trait IcalUseCase {
    /// 新しいicalを登録する
    fn register(&self, arg: RegisterArgs) -> Result<()>;
    /// ある所有者(user)のicalリストを返す
    fn show_icals(&self, owner: String) -> Result<Vec<Ical>>;
    /// ある所有者のイベントリストを返す(ical_id, eventlist)
    fn show_user_ical_events(&self, owner: String) -> Result<HashMap<Ulid, Vec<IcalEvent>>>;
    /// あるicalのイベントリストを返す
    fn show_icals_events(&self, ical_id: Ulid) -> Result<Vec<IcalEvent>>;
    /// icalを削除する
    fn delete_ical(&self, id: Ulid) -> Result<()>;
    fn update(&self, arg: UpdateArgs) -> Result<()>;
}

pub struct RegisterArgs {
    pub title: Option<String>,
    pub url: Url,
    pub owner: String,
    pub channel: String,
    pub notification_time: Vec<Duration>,
}
