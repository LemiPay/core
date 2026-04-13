use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::group::group_wallet::{GroupWallet, NewGroupWallet};
use crate::repositories::traits::group_wallet_repo::GroupWalletRepository;
use crate::schema::group_wallet;

pub struct DieselGroupWalletRepository {
    db: Db,
}

impl DieselGroupWalletRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl GroupWalletRepository for DieselGroupWalletRepository {
    fn create(&self, new_group_wallet: NewGroupWallet) -> Result<GroupWallet, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = diesel::insert_into(group_wallet::table)
            .values(&new_group_wallet)
            .returning(GroupWallet::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    }

    fn get_wallets_by_group(&self, group_id: Uuid) -> Result<Vec<GroupWallet>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = group_wallet::table
            .filter(group_wallet::group_id.eq(group_id))
            .load::<GroupWallet>(&mut conn)?;
        Ok(result)
    }

    fn get_wallet_by_group_and_currency(
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
}
