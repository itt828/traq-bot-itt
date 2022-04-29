use crate::apis::message::post_message;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::{Job, JobScheduler};

static NOIMAGE: &str = "https://s.yimg.jp/images/weather/common/noimage.gif";
static GTIBOT: &str = "";

pub async fn earthquake() -> Result<(), Box<dyn std::error::Error>> {
    let last_earthquake: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let re = Regex::new(
        r"^https://weather-pctr.c.yimg.jp/t/weather-img/earthquake/(?P<time>\d{14})/.*$",
    )
    .unwrap();
    let re = Arc::new(re);

    let sched = JobScheduler::new()?;
    println!("ok");
    sched.add(Job::new_async("0/5 * * * * *", move |_uuid, _l| {
        let last_earthquake_clone = last_earthquake.clone();
        let re_clone = re.clone();
        Box::pin(async move {
            let x = scrape_from_yahoo().await;
            let mut last_earthquake_clone_lock = last_earthquake_clone.lock().unwrap();
            match &mut *last_earthquake_clone_lock {
                Some(s) => {
                    if *s != x && &x != NOIMAGE {
                        *s = x;
                        if let Some(matches) = re_clone.captures(s) {
                            let time = &matches["time"];
                            let msg = format!(
                                r"## 地震発生
                                https://typhoon.yahoo.co.jp/weather/jp/earthquake/{}.html",
                                time
                            );

                            tokio::spawn(async move {
                                post_message(&msg, GTIBOT).await;
                            });
                        }
                    }
                }
                None => {
                    if &x != NOIMAGE {
                        *last_earthquake_clone_lock = Some(x.clone());
                    }
                }
            };
        })
    })?)?;
    sched.start();
    Ok(())
}
//return url of the latest earthquake image
pub async fn scrape_from_yahoo() -> String {
    let resp = reqwest::get("https://typhoon.yahoo.co.jp/weather/jp/earthquake/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let selector = Selector::parse("div.tabView_content#earthquake-01").unwrap();
    let doc = Html::parse_document(&resp);
    let x = doc
        .select(&selector)
        .next()
        .unwrap()
        .children()
        .collect::<Vec<_>>()[1]
        .value()
        .as_element()
        .unwrap()
        .attr("src")
        .unwrap();
    let x = String::from(x);
    x
}
