use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::{UserWalletDetails, UserWalletWithTickerDetails};
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::UserId;

pub trait UserWalletRepository: Send + Sync {
    fn save(&self, wallet: &UserWallet) -> Result<(), RepoError>;
    fn find_by_id(&self, id: UserWalletId) -> Result<Option<UserWallet>, RepoError>;
    fn find_by_address_and_currency(
        &self,
        address: &str,
        currency: CurrencyId,
    ) -> Result<Option<UserWallet>, RepoError>;
    fn find_owner_of_address(&self, address: &str) -> Result<Option<UserId>, RepoError>;

    /// Atomic transfer between two user wallets that share the same currency.
    /// The repo enforces "balance >= amount" inside the DB transaction so we
    /// don't run into TOCTOU issues between read and write.
    fn transfer(
        &self,
        sender: UserWalletId,
        receiver: UserWalletId,
        amount: &Money,
    ) -> Result<(), RepoError>;

    fn get_details(&self, id: UserWalletId) -> Result<Option<UserWalletDetails>, RepoError>;
    fn list_with_ticker_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserWalletWithTickerDetails>, RepoError>;
}
