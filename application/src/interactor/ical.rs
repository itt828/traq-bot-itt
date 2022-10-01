use crate::ical_reader::IcalReader;
use crate::repository::ical::{CreateArgs, FindArgs, IcalRepository, UpdateArgs};
use crate::usecase::ical::{IcalUseCase, RegisterArgs};
use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use domain::ical::ical::Ical;
use domain::ical::ical_event::IcalEvent;
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use ulid::Ulid;

#[derive(new)]
pub struct IcalInteractor<R: IcalRepository, S: IcalReader> {
    repository: Arc<R>,
    reader: Arc<S>,
}

#[async_trait]
impl<R: IcalRepository, S: IcalReader> IcalUseCase for IcalInteractor<R, S> {
    async fn register(&self, arg: RegisterArgs) -> Result<()> {
        let new_ical = CreateArgs {
            title: arg.title,
            url: arg.url,
            owner: arg.owner,
            channel: arg.channel,
            id: Ulid::new(),
            notification_time: arg.notification_time,
        };
        self.repository.create(new_ical).await?;
        Ok(())
    }
    async fn show_icals(&self, owner: String) -> Result<Vec<Ical>> {
        let find_arg = FindArgs {
            owner: Some(owner),
            title: None,
            id: None,
        };
        self.repository.find(&find_arg).await
    }
    async fn show_icals_events(&self, ical_id: Ulid) -> Result<Vec<IcalEvent>> {
        let ical = &self
            .repository
            .find(&FindArgs {
                title: None,
                id: Some(ical_id),
                owner: None,
            })
            .await?[0];
        self.reader.read(ical.url.clone(), None, None, None).await
    }
    async fn show_user_ical_events(&self, owner: String) -> Result<HashMap<Ulid, Vec<IcalEvent>>> {
        let icals = self.show_icals(owner).await?;
        let events = icals
            .iter()
            .map(|v| async { (v.id, self.show_icals_events(v.id).await.unwrap()) });
        let events = join_all(events)
            .await
            .into_iter()
            .collect::<HashMap<Ulid, Vec<IcalEvent>>>();
        Ok(events)
    }
    async fn delete_ical(&self, id: Ulid) -> Result<()> {
        self.repository.delete(id).await?;
        Ok(())
    }
    async fn update(&self, arg: UpdateArgs) -> Result<()> {
        self.repository.update(arg).await
    }
}
