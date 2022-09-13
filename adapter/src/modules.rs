use crate::{repository::ical::IcalRepository, sqlhandler::ical::IcalSqlHandler};

pub struct RepositoriesModule {
    ical_repository: IcalSqlHandler,
}

pub trait RepositoriesModuleExt {
    type IcalRepo: IcalRepository;
    fn ical_repository(&self) -> &Self::IcalRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type IcalRepo = IcalSqlHandler;
    fn ical_repository(&self) -> &Self::IcalRepo {
        &self.ical_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let ical_repository = IcalSqlHandler::new(db.clone());
        Self { ical_repository }
    }
}
