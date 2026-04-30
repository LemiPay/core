use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::TransactionDetails;
use crate::domain::group::GroupId;
use crate::domain::treasury::{NewTransaction, TransactionId};

pub trait TransactionRepository: Send + Sync {
    /// Atomic deposit from a user wallet to a group wallet.
    /// The repo handles balance validation inside a DB transaction.
    fn create_user_to_group_deposit(
        &self,
        new_tx: NewTransaction,
    ) -> Result<TransactionDetails, RepoError>;

    fn list_by_group(&self, group_id: GroupId) -> Result<Vec<TransactionDetails>, RepoError>;
    fn find_by_id(&self, id: TransactionId) -> Result<Option<TransactionDetails>, RepoError>;
}
