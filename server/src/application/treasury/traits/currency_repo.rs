use crate::application::common::repo_error::RepoError;
use crate::domain::treasury::CurrencyId;

#[derive(Debug, Clone)]
pub struct CurrencyInfo {
    pub id: CurrencyId,
    pub token_address: String,
    pub decimals: i16,
}

pub trait CurrencyRepository: Send + Sync {
    fn find_id_by_ticker(&self, ticker: &str) -> Result<Option<CurrencyId>, RepoError>;
    fn find_id_by_token_currency_id(
        &self,
        token_currency_id: &str,
    ) -> Result<Option<CurrencyId>, RepoError>;
    fn find_by_id(&self, id: CurrencyId) -> Result<Option<CurrencyInfo>, RepoError>;
}
