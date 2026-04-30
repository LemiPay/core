use diesel::prelude::*;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::traits::currency_repo::CurrencyRepository;
use crate::domain::treasury::CurrencyId;
use crate::infrastructure::db::{
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselCurrencyRepository {
    db: DbPool,
}

impl DieselCurrencyRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

impl CurrencyRepository for DieselCurrencyRepository {
    fn find_id_by_ticker(&self, ticker: &str) -> Result<Option<CurrencyId>, RepoError> {
        let mut conn = self.get_conn()?;

        let id = schema::currency::table
            .filter(schema::currency::ticker.eq(ticker))
            .select(schema::currency::currency_id)
            .first::<Uuid>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(id.map(CurrencyId))
    }
}
