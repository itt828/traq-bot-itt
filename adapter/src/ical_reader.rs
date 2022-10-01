use anyhow::Result;
use application::ical_reader::IcalReader;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use ical::{parser::ical::component::IcalEvent, IcalParser};
use reqwest;
use url::Url;

pub struct IcalReaderImpl;

#[async_trait]
impl IcalReader for IcalReaderImpl {
    async fn read(
        &self,
        url: Url,
        start_time: Option<DateTime<Local>>,
        end_time: Option<DateTime<Local>>,
        max: Option<u32>,
    ) -> Result<Vec<domain::ical::ical_event::IcalEvent>> {
        let ical = reqwest::get(url).await?.bytes().await?;
        let parsed_ical = IcalParser::new(&ical[..]).next().unwrap()?.events;
        let ical_events = parsed_ical.iter().map(|v| ical_converter(v)).filter(|v| {
            let mut st_is_ok = true;
            let mut et_is_ok = true;
            if let Some(st) = start_time {
                if st > v.start_dt {
                    st_is_ok = false;
                }
            }
            if let Some(et) = end_time {
                if v.end_dt <= et {
                    et_is_ok = false;
                }
            }
            st_is_ok && et_is_ok
        });
        let ical_events = if let Some(v) = max {
            ical_events.take(v as usize).collect::<Vec<_>>()
        } else {
            ical_events.collect::<Vec<_>>()
        };
        Ok(ical_events)
    }
}

fn ical_converter(ical_event: &IcalEvent) -> domain::ical::ical_event::IcalEvent {
    let mut domain_ical_event = domain::ical::ical_event::IcalEvent {
        summary: "".to_owned(),
        start_dt: Local::now(),
        end_dt: Local::now(),
        description: None,
        uid: None,
        location: None,
        rrule: None,
    };
    for property in ical_event.properties.iter() {
        match &*property.name {
            "SUMMERY" => {
                domain_ical_event.summary = property.value.as_ref().unwrap().clone();
            }
            "DESCRIPTION" => {
                domain_ical_event.description = Some(property.value.as_ref().unwrap().clone());
            }
            "DTSTART" => {
                //domain_ical_event.start_dt = property.value.as_ref().unwrap().clone();
            }
            "DTEND" => {}
            "LOCATION" => {
                domain_ical_event.location = Some(property.value.as_ref().unwrap().clone());
            }
            "RRULE" => {
                domain_ical_event.rrule = Some(property.value.as_ref().unwrap().clone());
            }
            "UID" => {
                domain_ical_event.uid = Some(property.value.as_ref().unwrap().clone());
            }
            _ => unimplemented!(),
        }
    }
    domain_ical_event
}
