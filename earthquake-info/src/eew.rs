use crate::error::*;
use crate::models::eew::Eew;
use chrono::prelude::*;
use reqwest;

pub async fn get_eew(dt: chrono::DateTime<FixedOffset>) -> Result<Eew> {
    println!("{}", dt);
    let new_eew = {
        let body = reqwest::get(format!(
            "http://www.kmoni.bosai.go.jp/webservice/hypo/eew/{}.json",
            dt.format("%Y%m%d%H%M%S")
        ))
        .await?
        .text()
        .await?;
        let v = serde_json::from_str::<Eew>(&body);
        v
    }?;
    Ok(new_eew)
}
