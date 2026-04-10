use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::user_wallet::{NewUserWallet, UserWallet};
use crate::repositories::traits::user_wallet_repo::UserWalletRepository;
use crate::schema::user_wallet;

pub struct DieselUserWalletRepository {
    db: Db,
}

impl DieselUserWalletRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserWalletRepository for DieselUserWalletRepository {
    fn create(&self, user_wallet: NewUserWallet) -> Result<UserWallet, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = diesel::insert_into(user_wallet::table)
            .values(&user_wallet)
            .returning(UserWallet::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    }

    fn get_balance_by_address(&self, address: &str) -> Result<BigDecimal, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_wallet::table
            .filter(user_wallet::address.eq(address))
            .select(user_wallet::balance)
            .first::<BigDecimal>(&mut conn)?;

        Ok(result)
    }

    fn add_money_to_wallet(&self, address: &str, amount: BigDecimal) -> Result<Uuid, DbError> {
        let mut conn = self.db.get_conn()?;

        let updated_wallet_id =
            diesel::update(user_wallet::table.filter(user_wallet::address.eq(address)))
                .set(user_wallet::balance.eq(user_wallet::balance + amount))
                .returning(user_wallet::user_id)
                .get_result::<Uuid>(&mut conn)?;

        Ok(updated_wallet_id)
    }

    fn take_money_by_address(
        &self,
        target_address: &str,
        amount: BigDecimal,
    ) -> Result<BigDecimal, DbError> {
        let mut conn = self.db.get_conn()?;

        let new_balance = diesel::update(
            user_wallet::table
                .filter(user_wallet::address.eq(target_address))
                .filter(user_wallet::balance.ge(amount.clone())),
        )
        .set(user_wallet::balance.eq(user_wallet::balance - amount))
        .returning(user_wallet::balance)
        .get_result::<BigDecimal>(&mut conn)?;

        Ok(new_balance)
    }
    fn make_transfer_between_addresses(
        &self,
        sender_address: &str,
        receiver_address: &str,
        amount: BigDecimal,
    ) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;

        let success = conn.transaction::<bool, DbError, _>(|this_conn| {
            diesel::update(
                user_wallet::table
                    .filter(user_wallet::address.eq(sender_address))
                    //filtro para el que el balance sea mayor a amount sino pincha porque no encuentra tupla
                    .filter(user_wallet::balance.ge(amount.clone())),
            )
            .set(user_wallet::balance.eq(user_wallet::balance - amount.clone()))
            .returning(user_wallet::address)
            .get_result::<String>(this_conn)?;

            diesel::update(user_wallet::table.filter(user_wallet::address.eq(receiver_address)))
                .set(user_wallet::balance.eq(user_wallet::balance + amount))
                .returning(user_wallet::address)
                .get_result::<String>(this_conn)?;

            Ok(true)
        })?;

        Ok(success)
    }

    fn verify_user_owns_wallet(
        &self,
        target_user_id: Uuid,
        target_address: &str,
    ) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;

        let wallet = user_wallet::table
            .filter(user_wallet::address.eq(target_address))
            .filter(user_wallet::user_id.eq(target_user_id))
            .select(user_wallet::address)
            .first::<String>(&mut conn)
            .optional()?;

        Ok(wallet.is_some())
    }

    fn get_user_address(&self, target_user_id: Uuid) -> Result<String, DbError> {
        let mut conn = self.db.get_conn()?;

        let address = user_wallet::table
            .filter(user_wallet::user_id.eq(target_user_id))
            .select(user_wallet::address)
            .first::<String>(&mut conn)?;

        Ok(address)
    }
}
