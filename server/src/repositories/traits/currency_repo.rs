use crate::data::error::DbError;
use uuid::Uuid;

pub trait CurrencyRepository: Send + Sync {
    fn check_if_currency_exist(&self, ticker: String) -> Result<Uuid, DbError>;
}
