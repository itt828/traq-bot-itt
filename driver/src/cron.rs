use crate::format::eew_format;
use anyhow::Result;
use earthquake_info::{earthquake::earthquake, models::earthquake::Earthquake};
use earthquake_info::{eew::get_new_eew, models::eew::Eew};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use traq::bot::Bot;

static GPS_EARTHQUAKE: &'static str = "971ff17d-9bc8-443d-bf05-30afc09be379";

pub fn cron(
    bot: Arc<Bot>,
    last_earthquake: Arc<tokio::sync::Mutex<Option<Earthquake>>>,
    last_eew: Arc<tokio::sync::Mutex<Option<(Eew, String)>>>,
) -> Result<()> {
    let sched = JobScheduler::new()?;
    let bcl = bot.clone();
    sched.add(Job::new("0/5 * * * * *", move |_, _| {
        let leq = last_earthquake.clone();
        let bot_cl = bcl.clone();
        tokio::spawn(async move {
            let f = |x: Earthquake| async move {
                let x = format!(
                    r"## 地震発生
- 発生時刻: **{}**
- 震源地: **{}**
- 最大震度: **{}**
- マグニチュード: **{}**
- 深さ: **{}**
- 情報: **{}**
https://typhoon.yahoo.co.jp/weather/jp/earthquake/{}.html

                    ",
                    x.time,
                    x.hypocenter,
                    x.max_seismic_intensity,
                    x.magnitude,
                    x.depth,
                    x.info,
                    x.url_time.unwrap()
                );
                bot_cl
                    .post_message(GPS_EARTHQUAKE, &x, false)
                    .await
                    .unwrap();
                Ok(())
            };
            earthquake(&mut *leq.lock().await, f).await.unwrap();
        });
    })?)?;
    sched.add(Job::new("1/1 * * * * *", move |_, _| {
        let leew = last_eew.clone();
        let bot_cl = bot.clone();
        tokio::spawn(async move {
            let new_eew = get_new_eew().await.unwrap();
            let leew = &mut *leew.lock().await;
            if new_eew.result.message == "" {
                let mut last_msg_id = String::new();
                match leew {
                    Some(v) => {
                        if v.0.report_id != new_eew.report_id {
                            let resp = bot_cl
                                .post_message(GPS_EARTHQUAKE, &eew_format(&new_eew), false)
                                .await
                                .unwrap();
                            last_msg_id = resp.id.clone();
                        } else {
                            if v.0.report_num != new_eew.report_num {
                                bot_cl
                                    .edit_message(&v.1, &eew_format(&new_eew), false)
                                    .await
                                    .unwrap();
                            }
                            last_msg_id = v.1.clone();
                        }
                    }
                    None => {
                        let resp = bot_cl
                            .post_message(GPS_EARTHQUAKE, &eew_format(&new_eew), false)
                            .await
                            .unwrap();
                        last_msg_id = resp.id.clone();
                    }
                };
                *leew = Some((new_eew.clone(), last_msg_id.to_string()));
            } else {
                *leew = Some((new_eew.clone(), "".to_string()));
            }
        });
    })?)?;

    sched.start()?;
    Ok(())
}
