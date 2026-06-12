use diesel::prelude::*;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::traits::currency_repo::{CurrencyInfo, CurrencyRepository};
use crate::domain::treasury::CurrencyId;
use crate::infrastructure::db::{
    models::treasury::CurrencyModel,
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

    fn find_id_by_token_currency_id(
        &self,
        token_currency_id: &str,
    ) -> Result<Option<CurrencyId>, RepoError> {
        let mut conn = self.get_conn()?;

        let id = schema::currency::table
            .filter(schema::currency::token_currency_id.eq(Some(token_currency_id)))
            .select(schema::currency::currency_id)
            .first::<Uuid>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(id.map(CurrencyId))
    }

    fn find_by_id(&self, id: CurrencyId) -> Result<Option<CurrencyInfo>, RepoError> {
        let mut conn = self.get_conn()?;

        let model: Option<CurrencyModel> = schema::currency::table
            .filter(schema::currency::currency_id.eq(id.0))
            .select(CurrencyModel::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(model.map(|m| CurrencyInfo {
            id: CurrencyId(m.currency_id),
            token_address: m.token_address,
            decimals: m.decimals,
        }))
    }
}
