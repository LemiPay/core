use crate::data::error::DbError;
use crate::errors::app_error::AppError;
use crate::handlers::group_wallet::NewGroupWalletRequest;
use crate::models::group::group_wallet::{GroupWallet, NewGroupWallet};
use crate::repositories::traits::currency_repo::CurrencyRepository;
use crate::repositories::traits::group_wallet_repo::GroupWalletRepository;
use diesel::result::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GroupWalletService {
    group_wallet_repo: Arc<dyn GroupWalletRepository>,
    currency_repo: Arc<dyn CurrencyRepository>,
}

impl GroupWalletService {
    pub fn new(
        group_wallet_repo: Arc<dyn GroupWalletRepository>,
        currency_repo: Arc<dyn CurrencyRepository>,
    ) -> Self {
        Self {
            group_wallet_repo,
            currency_repo,
        }
    }

    pub fn create_wallet(
        &self,
        request: NewGroupWalletRequest,
        group_id: Uuid,
    ) -> Result<GroupWallet, AppError> {
        let currency_id = match self
            .currency_repo
            .get_currency_id_by_ticker(request.currency_ticker)
        {
            Ok(currency_id) => currency_id,
            Err(DbError::Diesel(Error::NotFound)) => {
                return Err(AppError::BadRequest("That currency doesn't exist".into()));
            }
            Err(err) => return Err(AppError::Db(err)),
        };

        let existing = self
            .group_wallet_repo
            .get_wallet_by_group_and_currency(group_id, currency_id)
            .map_err(AppError::Db)?;

        if existing.is_some() {
            return Err(AppError::BadRequest(
                "The group already has a wallet for this currency".into(),
            ));
        }

        let address_taken = self
            .group_wallet_repo
            .get_wallet_by_address_and_currency(&request.address, currency_id)
            .map_err(AppError::Db)?;

        if address_taken.is_some() {
            return Err(AppError::BadRequest(
                "That address is already registered for this currency".into(),
            ));
        }

        let new_wallet = NewGroupWallet {
            address: request.address,
            group_id,
            currency_id,
        };

        self.group_wallet_repo
            .create(new_wallet)
            .map_err(AppError::Db)
    }

    pub fn get_wallets_by_group(&self, group_id: Uuid) -> Result<Vec<GroupWallet>, AppError> {
        self.group_wallet_repo
            .get_wallets_by_group(group_id)
            .map_err(AppError::Db)
    }
}
