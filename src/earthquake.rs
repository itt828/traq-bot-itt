use crate::{apis::message::post_message, models::earthquake::Earthquake};
use regex::Regex;
use scraper::{Html, Selector};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tokio_cron_scheduler::{Job, JobScheduler};

static GTIBOT: &str = "0043558c-6efb-4a01-8a21-fcb171190f64";

pub async fn earthquake() -> Result<(), Box<dyn std::error::Error>> {
    let last_earthquake: Arc<Mutex<Option<Earthquake>>> = Arc::new(Mutex::new(None));

    let sched = JobScheduler::new()?;
    sched.add(Job::new_async("0/5 * * * * *", move |_uuid, _l| {
        let last_earthquake_clone = last_earthquake.clone();
        Box::pin(async move {
            let new_earthquake = scrape_from_yahoo().await;
            let mut last_earthquake_clone_lock = last_earthquake_clone.lock().unwrap();
            match (&*last_earthquake_clone_lock, &new_earthquake.url_time) {
                (Some(x), Some(y)) => match &x.url_time {
                    Some(z) => {
                        if z != y {
                            *last_earthquake_clone_lock = Some(new_earthquake.clone());
                            let msg = format!(
                                r"## 地震発生
- 発生時刻: **{}**
- 震源地: **{}**
- 最大震度: **{}**
- マグニチュード: **{}**
- 深さ: **{}**
- 情報: **{}**
https://typhoon.yahoo.co.jp/weather/jp/earthquake/{}.html",
                                &new_earthquake.time,
                                &new_earthquake.hypocenter,
                                &new_earthquake.max_seismic_intensity,
                                &new_earthquake.magnitude,
                                &new_earthquake.depth,
                                &new_earthquake.info,
                                &new_earthquake.url_time.unwrap()
                            );

                            tokio::spawn(async move {
                                post_message(&msg, GTIBOT).await;
                            });
                        }
                    }
                    None => {
                        *last_earthquake_clone_lock = Some(new_earthquake);
                        tokio::spawn(async {
                            post_message("a", "a").await;
                        });
                    }
                },
                (None, _) => {
                    *last_earthquake_clone_lock = Some(new_earthquake);
                }
                (Some(_), None) => {}
            };
        })
    })?)?;
    sched.start();
    Ok(())
}
pub fn extract_url_time(url: &str) -> Option<String> {
    let re = Regex::new(
        r"^https://weather-pctr.c.yimg.jp/t/weather-img/earthquake/(?P<time>\d{14})/.*$",
    )
    .unwrap();
    if let Some(matches) = &re.captures(url) {
        Some(String::from(&matches["time"]))
    } else {
        None
    }
}
//return url of the latest earthquake image
pub async fn scrape_from_yahoo() -> Earthquake {
    let resp = reqwest::get("https://typhoon.yahoo.co.jp/weather/jp/earthquake/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let doc = Html::parse_document(&resp);
    let url_time = {
        let selector = Selector::parse("#earthquake-01 > img:nth-child(1)").unwrap();
        let url = doc
            .select(&selector)
            .nth(0)
            .unwrap()
            .value()
            .attr("src")
            .unwrap();
        extract_url_time(url)
    };
    let time = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .deref()
    };
    let time = String::from(time);
    let hypocenter = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .deref()
    };
    let hypocenter = String::from(hypocenter);
    let max_seismic_intensity = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .deref()
    };
    let max_seismic_intensity = String::from(max_seismic_intensity);
    let magnitude = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .deref()
    };
    let magnitude = String::from(magnitude);
    let depth = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(5) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .deref()
    };
    let depth = String::from(depth);
    let info = {
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(7) > td:nth-child(2) > small:nth-child(1)").unwrap();
        doc.select(&selector)
            .next()
            .unwrap()
            .children()
            .nth(0)
            .unwrap()
            .value()
            .as_text()
            .unwrap()
            .trim()
    };
    let info = String::from(info);
    Earthquake {
        url_time,
        time,
        hypocenter,
        max_seismic_intensity,
        magnitude,
        depth,
        info,
    }
}
