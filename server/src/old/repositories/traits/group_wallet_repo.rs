use crate::data::error::DbError;
use crate::models::group::group_wallet::{GroupWallet, NewGroupWallet};
use uuid::Uuid;

pub trait GroupWalletRepository: Send + Sync {
    fn create(&self, group_wallet: NewGroupWallet) -> Result<GroupWallet, DbError>;
    fn get_wallets_by_group(&self, group_id: Uuid) -> Result<Vec<GroupWallet>, DbError>;
    fn get_wallet_by_group_and_currency(
        &self,
        group_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<GroupWallet>, DbError>;
    fn get_wallet_by_address_and_currency(
        &self,
        address: &str,
        currency_id: Uuid,
    ) -> Result<Option<GroupWallet>, DbError>;
}
