use crate::data::error::DbError;
use crate::models::user_wallet::{NewUserWallet, UserWallet};
use bigdecimal::BigDecimal;
use uuid::Uuid;

pub trait UserWalletRepository: Send + Sync {
    fn create(&self, user: NewUserWallet) -> Result<UserWallet, DbError>;
    fn get_balance_by_address(&self, address: &str) -> Result<BigDecimal, DbError>;
    fn add_money_to_wallet(&self, user_id: Uuid, amount: BigDecimal) -> Result<Uuid, DbError>;
    fn take_money_by_address(&self, address: &str) -> Result<BigDecimal, DbError>;
    fn make_transfer_between_addresses(
        &self,
        sender_address: &str,
        receiver_address: &str,
    ) -> Result<bool, DbError>;
    fn verify_user_owns_wallet(&self, user_id: Uuid, address: &str) -> Result<bool, DbError>;
    /**
    # This method will be deprecated the moment we allow multiple wallets per user
    */
    fn get_user_address(&self, user_id: Uuid) -> Result<String, DbError>;
}
