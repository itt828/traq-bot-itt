use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use domain::ical::ical::Ical;
use ulid::Ulid;
use url::Url;

#[async_trait]
pub trait IcalRepository: Send + Sync + 'static {
    async fn create(&self, arg: CreateArgs) -> Result<()>;
    async fn update(&self, arg: UpdateArgs) -> Result<()>;
    async fn find(&self, arg: &FindArgs) -> Result<Vec<Ical>>;
    async fn delete(&self, id: Ulid) -> Result<()>;
}

pub struct CreateArgs {
    /// ulid
    pub id: Ulid,
    /// タイトルを入れるかは任意
    pub title: Option<String>,
    /// icalのurl
    pub url: Url,
    pub owner: String,
    /// icalを投稿するチャンネル
    pub channel: String,
    /// 通知する時間
    pub notification_time: Vec<Duration>,
}
pub struct UpdateArgs {
    /// タイトル(外側の`Option`が変更ありかどうかを示す)
    pub title: Option<Option<String>>,
    ///
    pub url: Option<Url>,
    ///
    pub channel: Option<String>,
    ///
    pub notification_time: Option<Vec<String>>,
}

pub struct FindArgs {
    /// タイトルで検索
    pub title: Option<String>,
    /// idで検索
    pub id: Option<Ulid>,
    /// 所有者で検索
    pub owner: Option<String>,
}