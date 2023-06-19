use earthquake_info::{
    earthquake_yahoo::get_current_earthquake, models::earthquake_yahoo::EarthquakeYahoo,
};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::Job;

pub async fn earthquake_info_cron(cron_expr: &str) -> Job {
    let last_eq: Arc<Mutex<Option<EarthquakeYahoo>>> = Arc::new(Mutex::new(None));
    Job::new_async(cron_expr, move |_uuid, _l| {
        let last_eq_clone = last_eq.clone();
        Box::pin(async move {
            let new_eq = get_current_earthquake().await.unwrap();
            let mut last_eq_locked = last_eq_clone.lock().unwrap();
            match &*last_eq_locked {
                Some(eq) => {
                    // if eq != &new_eq {
                    println!("{:#?}", eq);
                    *last_eq_locked = Some(new_eq.clone());
                    // }
                }
                None => {
                    *last_eq_locked = Some(new_eq.clone());
                }
            };
        })
    })
    .unwrap()
}

// pub async fn fetch_earthquake() -> anyhow::Result<String> {
//     let eq = get_current_earthquake().await?;
//     Ok(String::new())
// }

fn format_earthquake(eq: EarthquakeYahoo) -> String {
    String::new()
}
