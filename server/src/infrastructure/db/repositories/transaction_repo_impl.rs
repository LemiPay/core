use diesel::prelude::*;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::transaction_repo::TransactionRepository;
use crate::domain::group::GroupId;
use crate::domain::treasury::{NewTransaction, TransactionId};
use crate::domain::user::UserId;
use crate::infrastructure::db::{
    models::treasury::{NewTransactionModel, TransactionModel, TransactionTypeModel},
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselTransactionRepository {
    db: DbPool,
}

impl DieselTransactionRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

fn model_to_details(model: TransactionModel) -> TransactionDetails {
    TransactionDetails {
        id: model.id,
        tx_hash: model.tx_hash,
        amount: model.amount,
        user_id: model.user_id,
        group_id: model.group_id,
        currency_id: model.currency_id,
        address: model.address,
        description: model.description,
        tx_type: model.tx_type.into(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

impl TransactionRepository for DieselTransactionRepository {
    fn create_user_to_group_deposit(
        &self,
        new_tx: NewTransaction,
    ) -> Result<TransactionDetails, RepoError> {
        let mut conn = self.get_conn()?;

        let amount_value = new_tx.amount.amount.clone();
        let currency_id = new_tx.currency_id.0;
        let user_id = new_tx.user_id.0;
        let group_id = new_tx.group_id.0;
        let address = new_tx.address.clone();
        let description = new_tx.description.clone();
        let tx_hash = new_tx.tx_hash.clone();
        let tx_type: TransactionTypeModel = new_tx.tx_type.into();

        let model = conn
            .transaction::<TransactionModel, diesel::result::Error, _>(|tx_conn| {
                let debited_user = diesel::update(
                    schema::user_wallet::table
                        .filter(schema::user_wallet::user_id.eq(user_id))
                        .filter(schema::user_wallet::address.eq(&address))
                        .filter(schema::user_wallet::currency_id.eq(currency_id))
                        .filter(schema::user_wallet::balance.ge(&amount_value)),
                )
                .set((
                    schema::user_wallet::balance.eq(schema::user_wallet::balance - &amount_value),
                    schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(tx_conn)?;

                if debited_user != 1 {
                    return Err(diesel::result::Error::NotFound);
                }

                let credited_group = diesel::update(
                    schema::group_wallet::table
                        .filter(schema::group_wallet::group_id.eq(group_id))
                        .filter(schema::group_wallet::currency_id.eq(currency_id)),
                )
                .set((
                    schema::group_wallet::balance.eq(schema::group_wallet::balance + &amount_value),
                    schema::group_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(tx_conn)?;

                if credited_group != 1 {
                    return Err(diesel::result::Error::NotFound);
                }

                let new_row = NewTransactionModel {
                    tx_hash,
                    amount: amount_value,
                    user_id,
                    group_id,
                    currency_id,
                    address,
                    description,
                    tx_type,
                };

                let inserted = diesel::insert_into(schema::transaction::table)
                    .values(&new_row)
                    .returning(TransactionModel::as_returning())
                    .get_result::<TransactionModel>(tx_conn)?;

                Ok(inserted)
            })
            .map_err(|_| RepoError::Insert)?;

        Ok(model_to_details(model))
    }

    fn list_by_group(&self, group_id: GroupId) -> Result<Vec<TransactionDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::transaction::table
            .filter(schema::transaction::group_id.eq(group_id.as_uuid()))
            .order(schema::transaction::created_at.asc())
            .select(TransactionModel::as_select())
            .load::<TransactionModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows.into_iter().map(model_to_details).collect())
    }

    fn list_by_user(&self, user_id: UserId) -> Result<Vec<TransactionDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::transaction::table
            .filter(schema::transaction::user_id.eq(user_id.as_uuid()))
            .order(schema::transaction::created_at.asc())
            .select(TransactionModel::as_select())
            .load::<TransactionModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows.into_iter().map(model_to_details).collect())
    }

    fn find_by_id(&self, id: TransactionId) -> Result<Option<TransactionDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::transaction::table
            .filter(schema::transaction::id.eq(id.0))
            .select(TransactionModel::as_select())
            .first::<TransactionModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(row.map(model_to_details))
    }
}
