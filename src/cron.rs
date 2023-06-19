mod earthquake_yahoo;
mod eew;
use anyhow::Result;
use tokio_cron_scheduler::JobScheduler;

use self::{earthquake_yahoo::earthquake_info_cron, eew::eew_info_cron};

pub async fn cron_jobs() -> Result<()> {
    let sched = JobScheduler::new().await?;
    sched
        .add(earthquake_info_cron("0/5 * * * * *").await)
        .await?;
    sched.add(eew_info_cron("0/1 * * * * *").await).await?;
    sched.start().await?;
    Ok(())
}
