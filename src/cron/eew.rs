use chrono::Local;
use earthquake_info::{eew::get_eew, models::eew::Eew};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::Job;

pub async fn eew_info_cron(cron_expr: &str) -> Job {
    let last_eew: Arc<Mutex<Option<Eew>>> = Arc::new(Mutex::new(None));
    Job::new_async(cron_expr, move |_uuid, _l| {
        let last_eew_clone = last_eew.clone();
        Box::pin(async move {
            let now = Local::now();
            let new_eew = get_eew(now).await.unwrap();
            let mut last_eew_locked = last_eew_clone.lock().unwrap();
            match &*last_eew_locked {
                Some(eew) => {
                    if eew != &new_eew {
                        println!("{:#?}", eew);
                        *last_eew_locked = Some(new_eew.clone());
                    }
                }
                None => {
                    *last_eew_locked = Some(new_eew.clone());
                }
            };
        })
    })
    .unwrap()
}
