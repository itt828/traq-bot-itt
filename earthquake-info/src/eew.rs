use crate::error::*;
use crate::models::eew::Eew;
use chrono::{prelude::*, Duration};
use reqwest;
pub async fn get_new_eew() -> Result<Eew> {
    let new_eew = {
        let dt = Local::now() + Duration::seconds(-2);
        let body = reqwest::get(format!(
            "http://www.kmoni.bosai.go.jp/webservice/hypo/eew/{}.json",
            dt.format("%Y%m%d%H%M%S")
        ))
        .await?
        .text()
        .await?;
        let v = serde_json::from_str::<Eew>(&body).unwrap();
        v
    };
    Ok(new_eew)
}
