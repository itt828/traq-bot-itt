mod earthquake_yahoo;
use anyhow::Result;
use tokio_cron_scheduler::JobScheduler;

use self::earthquake_yahoo::earthquake_info_cron;

pub async fn cron_jobs() -> Result<()> {
    let sched = JobScheduler::new().await?;
    sched.add(earthquake_info_cron().await).await?;
    sched.start().await?;
    Ok(())
}
