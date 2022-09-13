use crate::repository::ical::IcalRepository;
use anyhow::Result;

pub struct IcalSqlHandler;

impl IcalRepository for IcalSqlHandler {
    fn create(ical: domain::ical::ical::Ical) -> Result<()> {}
    fn delete(id: String) -> anyhow::Result<()> {}
    fn find_by_id(id: ulid::Ulid) -> anyhow::Result<Vec<domain::ical::ical::Ical>> {}
    fn find_by_owner(owner: String) -> anyhow::Result<domain::ical::ical::Ical> {}
    fn update(ical: domain::ical::ical::Ical) -> anyhow::Result<()> {}
}

impl IcalSqlHandler {
    pub fn new() -> Self {
        Self
    }
}
