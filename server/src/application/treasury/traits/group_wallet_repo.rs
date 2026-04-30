use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::GroupWalletDetails;
use crate::domain::group::GroupId;
use crate::domain::treasury::{CurrencyId, GroupWallet, GroupWalletId};

pub trait GroupWalletRepository: Send + Sync {
    fn save(&self, wallet: &GroupWallet) -> Result<(), RepoError>;
    fn find_by_id(&self, id: GroupWalletId) -> Result<Option<GroupWallet>, RepoError>;
    fn find_by_group_and_currency(
        &self,
        group_id: GroupId,
        currency: CurrencyId,
    ) -> Result<Option<GroupWallet>, RepoError>;
    fn find_by_address_and_currency(
        &self,
        address: &str,
        currency: CurrencyId,
    ) -> Result<Option<GroupWallet>, RepoError>;

    fn list_details_by_group(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<GroupWalletDetails>, RepoError>;
}
