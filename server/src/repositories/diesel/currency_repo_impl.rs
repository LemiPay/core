use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::currency::Currency;
use crate::repositories::traits::currency_repo::CurrencyRepository;
use crate::schema::currency;

pub struct DieselCurrencyRepository {
    db: Db,
}
impl DieselCurrencyRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
impl CurrencyRepository for DieselCurrencyRepository {
    fn check_if_currency_exist(&self, ticker: String) -> Result<Uuid, DbError> {
        let mut conn = self.db.get_conn()?;

        let currency_id = currency::table
            .filter(currency::ticker.eq(ticker))
            .select(currency::currency_id)
            .first::<Uuid>(&mut conn)?;

        Ok(currency_id)
    }
}
