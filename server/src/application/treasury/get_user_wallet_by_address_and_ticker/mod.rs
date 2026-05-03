use std::sync::Arc;

use crate::application::treasury::dto::UserWalletDetails;
use crate::application::treasury::traits::{
    currency_repo::CurrencyRepository, user_wallet_repo::UserWalletRepository,
};
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum GetUserWalletError {
    CurrencyNotFound,
    NotFound,
    NotOwner,
    Internal,
}

#[derive(Clone)]
pub struct GetUserWalletByAddressAndTickerUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
}

impl GetUserWalletByAddressAndTickerUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
        address: &str,
        ticker: &str,
    ) -> Result<UserWalletDetails, GetUserWalletError> {
        let currency = self
            .currency_repo
            .find_id_by_ticker(ticker)
            .map_err(|_| GetUserWalletError::Internal)?
            .ok_or(GetUserWalletError::CurrencyNotFound)?;

        let wallet = self
            .user_wallet_repo
            .find_by_address_and_currency(address, currency)
            .map_err(|_| GetUserWalletError::Internal)?
            .ok_or(GetUserWalletError::NotFound)?;

        if !wallet.is_owned_by(user_id) {
            return Err(GetUserWalletError::NotOwner);
        }

        self.user_wallet_repo
            .get_details(wallet.id)
            .map_err(|_| GetUserWalletError::Internal)?
            .ok_or(GetUserWalletError::NotFound)
    }
}
