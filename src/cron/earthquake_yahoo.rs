use anyhow::Ok;
use earthquake_info::{
    earthquake_yahoo::get_current_earthquake, models::earthquake_yahoo::EarthquakeYahoo,
};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::Job;
use traq::{
    apis::{channel_api::post_message, configuration::Configuration},
    models::PostMessageRequest,
};

use crate::GPS_EARTHQUAKE;

pub async fn earthquake_info_cron(
    cron_expr: &str,
    config: Arc<Configuration>,
) -> anyhow::Result<Job> {
    let last_eq: Arc<Mutex<Option<EarthquakeYahoo>>> = Arc::new(Mutex::new(None));
    Ok(Job::new_async(cron_expr, move |_uuid, _l| {
        let last_eq_clone = last_eq.clone();
        let config_clone = config.clone();
        Box::pin(async move {
            let new_eq = get_current_earthquake().await.unwrap();
            let mut post_message_flag = false;
            {
                let mut last_eq_locked = last_eq_clone.lock().unwrap();
                match &*last_eq_locked {
                    Some(eq) => {
                        if eq != &new_eq {
                            post_message_flag = true;
                            *last_eq_locked = Some(new_eq.clone());
                        }
                    }
                    None => {
                        *last_eq_locked = Some(new_eq.clone());
                    }
                };
            }
            if post_message_flag {
                earthquake_update_handler(&new_eq, config_clone).await;
            }
        })
    })?)
}

async fn earthquake_update_handler(eq: &EarthquakeYahoo, config: Arc<Configuration>) {
    let message = format!(
        r"## 地震発生
- 発生時刻: **{}**
- 震源地: **{}**
- 最大震度: **{}**
- マグニチュード: **{}**
- 深さ: **{}**
- 情報: **{}**
https://typhoon.yahoo.co.jp/weather/jp/earthquake/{}.html",
        eq.time,
        eq.hypocenter,
        eq.max_seismic_intensity,
        eq.magnitude,
        eq.depth,
        eq.info,
        eq.url_time.as_ref().unwrap()
    );
    let _ = post_message(
        &config,
        GPS_EARTHQUAKE,
        Some(PostMessageRequest {
            content: message,
            embed: None,
        }),
    )
    .await;
}
