use crate::error::*;
use crate::models::earthquake_yahoo::EarthquakeYahoo;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::ops::Deref;

pub async fn get_current_earthquake() -> Result<EarthquakeYahoo> {
    let new_earthquake = scrape_from_yahoo().await;
    Ok(new_earthquake)
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
pub async fn scrape_from_yahoo() -> EarthquakeYahoo {
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
        let selector = Selector::parse(r"table.yjw_table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > small:nth-child(1) > a:nth-child(1)" ).unwrap();
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
    EarthquakeYahoo {
        url_time,
        time,
        hypocenter,
        max_seismic_intensity,
        magnitude,
        depth,
        info,
    }
}
