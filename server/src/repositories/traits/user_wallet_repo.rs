use crate::data::error::DbError;
use crate::models::user_wallet::{NewUserWallet, UserWallet};
use bigdecimal::BigDecimal;
use uuid::Uuid;

pub trait UserWalletRepository: Send + Sync {
    fn make_transfer_between_wallets(
        &self,
        sender_address_id: Uuid,
        receiver_address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<bool, DbError>;
    fn verify_user_owns_wallet(&self, user_id: Uuid, address: &str) -> Result<bool, DbError>;
    fn create(&self, user_wallet: NewUserWallet) -> Result<UserWallet, DbError>;
    fn take_money_by_address(
        &self,
        address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, DbError>;
    fn add_money_to_wallet(
        &self,
        address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, DbError>;
    fn get_balance_by_address_and_currency(
        &self,
        address: &str,
        currency_id: Uuid,
    ) -> Result<BigDecimal, DbError>;
    fn get_wallet_id_by_address_and_currency(
        &self,
        address: &str,
        currency_id: Uuid,
    ) -> Result<Uuid, DbError>;
    fn get_wallet_info(&self, wallet_id: Uuid) -> Result<UserWallet, DbError>;
    fn get_owner_of_address(&self, address: &str) -> Result<Option<Uuid>, DbError>;
}
