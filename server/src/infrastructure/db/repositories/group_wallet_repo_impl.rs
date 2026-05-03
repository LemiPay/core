use diesel::prelude::*;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::GroupWalletDetails;
use crate::application::treasury::traits::group_wallet_repo::GroupWalletRepository;
use crate::domain::group::GroupId;
use crate::domain::treasury::{CurrencyId, GroupWallet, GroupWalletId, Money};
use crate::infrastructure::db::{
    models::treasury::{GroupWalletModel, NewGroupWalletModel},
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselGroupWalletRepository {
    db: DbPool,
}

impl DieselGroupWalletRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

fn model_to_domain(model: GroupWalletModel) -> GroupWallet {
    GroupWallet::rehydrate(
        GroupWalletId(model.id),
        model.address,
        GroupId(model.group_id),
        Money {
            amount: model.balance,
            currency: CurrencyId(model.currency_id),
        },
    )
}

fn model_to_details(model: GroupWalletModel) -> GroupWalletDetails {
    GroupWalletDetails {
        id: model.id,
        address: model.address,
        group_id: model.group_id,
        currency_id: model.currency_id,
        balance: model.balance,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

impl GroupWalletRepository for DieselGroupWalletRepository {
    fn save(&self, wallet: &GroupWallet) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_wallet = NewGroupWalletModel {
            id: wallet.id.0,
            address: wallet.address.clone(),
            group_id: wallet.group_id.0,
            currency_id: wallet.balance.currency.0,
            balance: wallet.balance.amount.clone(),
        };

        diesel::insert_into(schema::group_wallet::table)
            .values(&new_wallet)
            .on_conflict(schema::group_wallet::id)
            .do_update()
            .set((
                schema::group_wallet::balance.eq(&wallet.balance.amount),
                schema::group_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }

    fn find_by_id(&self, id: GroupWalletId) -> Result<Option<GroupWallet>, RepoError> {
        let mut conn = self.get_conn()?;
        let model = schema::group_wallet::table
            .filter(schema::group_wallet::id.eq(id.0))
            .select(GroupWalletModel::as_select())
            .first::<GroupWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(model.map(model_to_domain))
    }

    fn find_by_group_and_currency(
        &self,
        group_id: GroupId,
        currency: CurrencyId,
    ) -> Result<Option<GroupWallet>, RepoError> {
        let mut conn = self.get_conn()?;
        let model = schema::group_wallet::table
            .filter(schema::group_wallet::group_id.eq(group_id.0))
            .filter(schema::group_wallet::currency_id.eq(currency.0))
            .select(GroupWalletModel::as_select())
            .first::<GroupWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(model.map(model_to_domain))
    }

    fn find_by_address_and_currency(
        &self,
        address: &str,
        currency: CurrencyId,
    ) -> Result<Option<GroupWallet>, RepoError> {
        let mut conn = self.get_conn()?;
        let model = schema::group_wallet::table
            .filter(schema::group_wallet::address.eq(address))
            .filter(schema::group_wallet::currency_id.eq(currency.0))
            .select(GroupWalletModel::as_select())
            .first::<GroupWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(model.map(model_to_domain))
    }

    fn list_details_by_group(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<GroupWalletDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let models = schema::group_wallet::table
            .filter(schema::group_wallet::group_id.eq(group_id.0))
            .select(GroupWalletModel::as_select())
            .load::<GroupWalletModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(models.into_iter().map(model_to_details).collect())
    }
}
