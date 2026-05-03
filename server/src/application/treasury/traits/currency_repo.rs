use crate::application::common::repo_error::RepoError;
use crate::domain::treasury::CurrencyId;

pub trait CurrencyRepository: Send + Sync {
    fn find_id_by_ticker(&self, ticker: &str) -> Result<Option<CurrencyId>, RepoError>;
}
