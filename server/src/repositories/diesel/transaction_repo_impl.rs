use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::group::group_wallet::GroupWallet;
use crate::models::proposal::{MyProposalStatus, ProposalUpdate};
use crate::models::transaction::{NewTransaction, Transaction};
use crate::models::user::user_wallet::UserWallet;
use crate::repositories::traits::transaction_repo::TransactionRepository;
use crate::schema::{group_wallet, proposal, transaction, user_wallet};

pub struct DieselTransactionRepository {
    db: Db,
}

impl DieselTransactionRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl TransactionRepository for DieselTransactionRepository {
    fn create_deposit(&self, new_tx: NewTransaction) -> Result<Transaction, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<Transaction, DbError, _>(|conn| {
            let debited_rows = diesel::update(user_wallet::table)
                .filter(user_wallet::user_id.eq(new_tx.user_id))
                .filter(user_wallet::currency_id.eq(new_tx.currency_id))
                .filter(user_wallet::balance.ge(&new_tx.amount))
                .set(user_wallet::balance.eq(user_wallet::balance - &new_tx.amount))
                .execute(conn)?;

            if debited_rows != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }

            let credited_rows = diesel::update(group_wallet::table)
                .filter(group_wallet::group_id.eq(new_tx.group_id))
                .filter(group_wallet::currency_id.eq(new_tx.currency_id))
                .set(group_wallet::balance.eq(group_wallet::balance + &new_tx.amount))
                .execute(conn)?;

            if credited_rows != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }
            let tx = diesel::insert_into(transaction::table)
                .values(&new_tx)
                .returning(Transaction::as_returning())
                .get_result(conn)?;

            Ok(tx)
        });

        Ok(result?)
    }

    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<Transaction>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = transaction::table
            .filter(transaction::group_id.eq(group_id))
            .order(transaction::created_at.desc())
            .get_results::<Transaction>(&mut conn)?;
        Ok(result)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Transaction>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = transaction::table
            .filter(transaction::id.eq(id))
            .first::<Transaction>(&mut conn)
            .optional()?;
        Ok(result)
    }

    fn get_user_wallet(
        &self,
        user_id: Uuid,
        address: String,
        currency_id: Uuid,
    ) -> Result<Option<UserWallet>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_wallet::table
            .filter(user_wallet::user_id.eq(user_id))
            .filter(user_wallet::currency_id.eq(currency_id))
            .filter(user_wallet::address.eq(address.clone()))
            .first::<UserWallet>(&mut conn)
            .optional()?;
        Ok(result)
    }

    fn get_group_wallet(
        &self,
        group_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<GroupWallet>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = group_wallet::table
            .filter(group_wallet::group_id.eq(group_id))
            .filter(group_wallet::currency_id.eq(currency_id))
            .first::<GroupWallet>(&mut conn)
            .optional()?;
        Ok(result)
    }

    fn execute_withdraw(
        &self,
        proposal_id: Uuid,
        new_tx: NewTransaction,
    ) -> Result<Transaction, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<Transaction, DbError, _>(|conn| {
            let updated_proposals =
                diesel::update(proposal::table.filter(proposal::id.eq(proposal_id)))
                    .set(ProposalUpdate {
                        status: MyProposalStatus::Executed,
                    })
                    .execute(conn)?;

            if updated_proposals != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }

            let debited_group_wallet = diesel::update(group_wallet::table)
                .filter(group_wallet::group_id.eq(new_tx.group_id))
                .filter(group_wallet::currency_id.eq(new_tx.currency_id))
                .filter(group_wallet::balance.ge(&new_tx.amount))
                .set(group_wallet::balance.eq(group_wallet::balance - &new_tx.amount))
                .execute(conn)?;

            if debited_group_wallet != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }

            let credited_user_wallet = diesel::update(user_wallet::table)
                .filter(user_wallet::user_id.eq(new_tx.user_id))
                .filter(user_wallet::currency_id.eq(new_tx.currency_id))
                .set(user_wallet::balance.eq(user_wallet::balance + &new_tx.amount))
                .execute(conn)?;

            if credited_user_wallet != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }
            let tx = diesel::insert_into(transaction::table)
                .values(&new_tx)
                .returning(Transaction::as_returning())
                .get_result(conn)?;

            Ok(tx)
        })?;

        Ok(result)
    }
}
