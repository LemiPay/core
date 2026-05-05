use std::sync::Arc;

use crate::application::treasury::create_group_wallet::dto::{
    CreateGroupWalletInput, CreateGroupWalletOutput,
};
use crate::application::treasury::create_group_wallet::error::CreateGroupWalletError;
use crate::application::treasury::traits::{
    currency_repo::CurrencyRepository, group_wallet_repo::GroupWalletRepository,
};
use crate::domain::treasury::GroupWallet;

#[derive(Clone)]
pub struct CreateGroupWalletUseCase {
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
}

impl CreateGroupWalletUseCase {
    pub fn execute(
        &self,
        input: CreateGroupWalletInput,
    ) -> Result<CreateGroupWalletOutput, CreateGroupWalletError> {
        let currency_id = self
            .currency_repo
            .find_id_by_ticker(&input.currency_ticker)
            .map_err(|_| CreateGroupWalletError::Internal)?
            .ok_or(CreateGroupWalletError::CurrencyNotFound)?;

        let already_for_currency = self
            .group_wallet_repo
            .find_by_group_and_currency(input.group_id, currency_id)
            .map_err(|_| CreateGroupWalletError::Internal)?;
        if already_for_currency.is_some() {
            return Err(CreateGroupWalletError::GroupAlreadyHasCurrency);
        }

        let address_taken = self
            .group_wallet_repo
            .find_by_address_and_currency(&input.address, currency_id)
            .map_err(|_| CreateGroupWalletError::Internal)?;
        if address_taken.is_some() {
            return Err(CreateGroupWalletError::AddressAlreadyTaken);
        }

        let wallet = GroupWallet::new(input.address, input.group_id, currency_id)?;

        self.group_wallet_repo
            .save(&wallet)
            .map_err(|_| CreateGroupWalletError::Internal)?;

        let details = self
            .group_wallet_repo
            .list_details_by_group(input.group_id)
            .map_err(|_| CreateGroupWalletError::Internal)?
            .into_iter()
            .find(|w| w.id == wallet.id.0)
            .ok_or(CreateGroupWalletError::Internal)?;

        Ok(CreateGroupWalletOutput { wallet: details })
    }
}
