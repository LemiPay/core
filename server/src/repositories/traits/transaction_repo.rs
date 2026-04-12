use crate::data::error::DbError;
use crate::models::group::group_wallet::GroupWallet;
use crate::models::transaction::{NewTransaction, Transaction};
use crate::models::user::user_wallet::UserWallet;
use uuid::Uuid;

pub trait TransactionRepository: Send + Sync {
    fn create_deposit(&self, new_tx: NewTransaction) -> Result<Transaction, DbError>;

    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<Transaction>, DbError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Transaction>, DbError>;

    fn get_user_wallet(
        &self,
        user_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<UserWallet>, DbError>;

    fn get_group_wallet(
        &self,
        group_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<GroupWallet>, DbError>;
}
