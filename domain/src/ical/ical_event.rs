use chrono::{DateTime, Local};
#[derive(Debug)]
pub struct IcalEvent {
    /// イベントが開始される時間
    pub start_dt: DateTime<Local>,
    /// イベントが終了する時間
    pub end_dt: DateTime<Local>,
    /// イベントのタイトル
    pub summary: String,
    /// イベントの説明・本文
    pub description: Option<String>,
    /// イベントの一意なID
    pub uid: Option<String>,
    /// イベントの開催場所
    pub location: Option<String>,
    /// イベントの繰り返しのルール
    pub rrule: Option<String>,
}
