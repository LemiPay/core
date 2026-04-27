use crate::data::error::DbError;
use uuid::Uuid;

pub trait CurrencyRepository: Send + Sync {
    fn get_currency_id_by_ticker(&self, ticker: String) -> Result<Uuid, DbError>;
}
