use std::sync::Arc;

use crate::application::treasury::create_user_wallet::dto::{
    CreateUserWalletInput, CreateUserWalletOutput,
};
use crate::application::treasury::create_user_wallet::error::CreateUserWalletError;
use crate::application::treasury::traits::{
    currency_repo::CurrencyRepository, user_wallet_repo::UserWalletRepository,
};
use crate::domain::treasury::UserWallet;

#[derive(Clone)]
pub struct CreateUserWalletUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
}

impl CreateUserWalletUseCase {
    pub fn execute(
        &self,
        input: CreateUserWalletInput,
    ) -> Result<CreateUserWalletOutput, CreateUserWalletError> {
        let currency_id = self
            .currency_repo
            .find_id_by_ticker(&input.currency_ticker)
            .map_err(|_| CreateUserWalletError::Internal)?
            .ok_or(CreateUserWalletError::CurrencyNotFound)?;

        if let Some(owner_id) = self
            .user_wallet_repo
            .find_owner_of_address(&input.address)
            .map_err(|_| CreateUserWalletError::Internal)?
            && owner_id != input.user_id
        {
            return Err(CreateUserWalletError::AddressTakenByOtherUser);
        }

        let already_for_currency = self
            .user_wallet_repo
            .find_by_address_and_currency(&input.address, currency_id)
            .map_err(|_| CreateUserWalletError::Internal)?;

        if already_for_currency.is_some() {
            return Err(CreateUserWalletError::AddressAlreadyHasCurrency);
        }

        let wallet = UserWallet::new(input.address, input.user_id, currency_id)?;

        self.user_wallet_repo
            .save(&wallet)
            .map_err(|_| CreateUserWalletError::Internal)?;

        let details = self
            .user_wallet_repo
            .get_details(wallet.id)
            .map_err(|_| CreateUserWalletError::Internal)?
            .ok_or(CreateUserWalletError::Internal)?;

        Ok(CreateUserWalletOutput { wallet: details })
    }
}
