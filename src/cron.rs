use anyhow::Result;
use earthquake_info::{earthquake::earthquake, models::earthquake::Earthquake};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use traq::bot::Bot;

pub fn cron(
    bot: Arc<Bot>,
    last_earthquake: Arc<tokio::sync::Mutex<Option<Earthquake>>>,
) -> Result<()> {
    let sched = JobScheduler::new()?;
    sched.add(Job::new("0/5 * * * * *", move |_, _| {
        let leq = last_earthquake.clone();
        let bot_cl = bot.clone();
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
                    .post_message("971ff17d-9bc8-443d-bf05-30afc09be379", &x, true)
                    .await
                    .unwrap();
                Ok(())
            };
            earthquake(&mut *leq.lock().await, f).await.unwrap();
        });
    })?)?;

    sched.start()?;
    Ok(())
}
