mod earthquake_yahoo;
mod eew;
use std::sync::Arc;

use anyhow::Result;
use tokio_cron_scheduler::JobScheduler;
use traq::apis::configuration::Configuration;

use self::{earthquake_yahoo::earthquake_info_cron, eew::eew_info_cron};

pub async fn cron_jobs(config: Arc<Configuration>) -> Result<()> {
    let sched = JobScheduler::new().await?;
    sched
        .add(earthquake_info_cron("0/5 * * * * *", config.clone()).await)
        .await?;
    sched
        .add(eew_info_cron("0/1 * * * * *", config.clone()).await)
        .await?;
    sched.start().await?;
    Ok(())
}
