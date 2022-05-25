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
            let f = |x| async move {
                let x = format!("{:?}", x);
                bot_cl
                    .post_message("0043558c-6efb-4a01-8a21-fcb171190f64", &x, true)
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
