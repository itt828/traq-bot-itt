use crate::repository::ical::{CreateArgs, FindArgs, IcalRepository, UpdateArgs};
use crate::usecase::ical::{IcalUseCase, RegisterArgs};
use anyhow::Result;
use chrono::Duration;
use derive_new::new;
use domain::ical::ical::Ical;
use domain::ical::ical_event::IcalEvent;
use std::collections::HashMap;
use ulid::Ulid;
use url::Url;

#[derive(new)]
pub struct IcalInteractor<R: IcalRepository> {
    repository: R,
}

impl<R: IcalRepository> IcalUseCase for IcalInteractor<R> {
    fn register(&self, arg: RegisterArgs) -> Result<()> {
        let new_ical = CreateArgs {
            title: arg.title,
            url: arg.url,
            owner: arg.owner,
            channel: arg.channel,
            id: Ulid::new(),
            notification_time: arg.notification_time,
        };
        self.repository.create(new_ical)?;
        Ok(())
    }
    fn show_icals(&self, owner: String) -> Result<Vec<Ical>> {
        let find_arg = FindArgs {
            owner: Some(owner),
            title: None,
            id: None,
        };
        self.repository.find(&find_arg)
    }
    fn show_user_ical_events(&self, owner: String) -> Result<HashMap<Ulid, Vec<IcalEvent>>> {
        unimplemented!()
    }
    fn show_icals_events(&self, ical_id: Ulid) -> Result<Vec<IcalEvent>> {
        unimplemented!()
    }
    fn delete_ical(&self, id: Ulid) -> Result<()> {
        self.repository.delete(id)
    }
    fn update(&self, arg: UpdateArgs) -> Result<()> {
        self.repository.update(arg)
    }
}
