use anyhow::Result;
use application::repository::ical::IcalRepository;
use async_trait::async_trait;

pub struct IcalSqlHandler;

#[async_trait]
impl IcalRepository for IcalSqlHandler {
    async fn find(
        &self,
        arg: &application::repository::ical::FindArgs,
    ) -> Result<Vec<domain::ical::ical::Ical>> {
        todo!()
    }
    async fn create(&self, arg: application::repository::ical::CreateArgs) -> Result<()> {
        todo!()
    }
    async fn update(&self, arg: application::repository::ical::UpdateArgs) -> Result<()> {
        todo!()
    }
    async fn delete(&self, id: ulid::Ulid) -> Result<()> {
        todo!()
    }
}

impl IcalSqlHandler {
    pub fn new() -> Self {
        Self
    }
}
