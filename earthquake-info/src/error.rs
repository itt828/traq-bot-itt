use std::result::Result as StdResult;
use thiserror::Error;
use tokio_cron_scheduler::JobSchedulerError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    JobScheduleError(#[from] JobSchedulerError),
}
