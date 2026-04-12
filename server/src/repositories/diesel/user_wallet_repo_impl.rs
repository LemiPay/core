use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::user_wallet::{NewUserWallet, UserWallet, WalletWithTickerDb};
use crate::repositories::traits::user_wallet_repo::UserWalletRepository;
use crate::schema::{currency, user_wallet};

pub struct DieselUserWalletRepository {
    db: Db,
}

impl DieselUserWalletRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserWalletRepository for DieselUserWalletRepository {
    fn make_transfer_between_wallets(
        &self,
        sender_address_id: Uuid,
        receiver_address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;

        let success = conn.transaction::<bool, DbError, _>(|this_conn| {
            let sender_currency = diesel::update(
                user_wallet::table
                    .filter(user_wallet::id.eq(sender_address_id))
                    .filter(user_wallet::balance.ge(amount.clone())),
            )
            .set(user_wallet::balance.eq(user_wallet::balance - amount.clone()))
            .returning(user_wallet::currency_id)
            .get_result::<Uuid>(this_conn)?;

            diesel::update(
                user_wallet::table
                    .filter(user_wallet::id.eq(receiver_address_id))
                    .filter(user_wallet::currency_id.eq(sender_currency)),
            )
            .set(user_wallet::balance.eq(user_wallet::balance + amount))
            .returning(user_wallet::address)
            .get_result::<String>(this_conn)?;

            Ok(true)
        })?;

        Ok(success)
    }

    fn verify_user_owns_wallet(&self, user_id: Uuid, address: &str) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;

        let wallet = user_wallet::table
            .filter(user_wallet::address.eq(address))
            .filter(user_wallet::user_id.eq(user_id))
            .select(user_wallet::address)
            .first::<String>(&mut conn)
            .optional()?;

        Ok(wallet.is_some())
    }

    fn create(&self, user_wallet: NewUserWallet) -> Result<UserWallet, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = diesel::insert_into(user_wallet::table)
            .values(&user_wallet)
            .returning(UserWallet::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    }

    fn take_money_from_wallet(
        &self,
        address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, DbError> {
        let mut conn = self.db.get_conn()?;

        let updated_wallet = diesel::update(
            user_wallet::table
                .filter(user_wallet::id.eq(address_id))
                .filter(user_wallet::balance.ge(amount.clone())),
        )
        .set(user_wallet::balance.eq(user_wallet::balance - amount))
        .get_result::<UserWallet>(&mut conn)?;

        Ok(updated_wallet)
    }
    fn add_money_to_wallet(
        &self,
        address_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, DbError> {
        let mut conn = self.db.get_conn()?;

        let updated_wallet =
            diesel::update(user_wallet::table.filter(user_wallet::id.eq(address_id)))
                .set(user_wallet::balance.eq(user_wallet::balance + amount))
                .get_result::<UserWallet>(&mut conn)?;

        Ok(updated_wallet)
    }

    fn get_balance_by_address_and_currency(
        &self,
        address: &str,
        currency_id: Uuid,
    ) -> Result<BigDecimal, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_wallet::table
            .filter(user_wallet::address.eq(address))
            .filter(user_wallet::currency_id.eq(currency_id))
            .select(user_wallet::balance)
            .first::<BigDecimal>(&mut conn)?;
        Ok(result)
    }

    fn get_wallet_id_by_address_and_currency(
        &self,
        address: &str,
        currency_id: Uuid,
    ) -> Result<Option<Uuid>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = user_wallet::table
            .filter(user_wallet::address.eq(address))
            .filter(user_wallet::currency_id.eq(currency_id))
            .select(user_wallet::id)
            .first::<Uuid>(&mut conn)
            .optional()?;

        Ok(result)
    }
    fn get_wallet_info(&self, wallet_id: Uuid) -> Result<UserWallet, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_wallet::table
            .filter(user_wallet::id.eq(wallet_id))
            .get_result::<UserWallet>(&mut conn)?;
        Ok(result)
    }

    fn get_owner_of_address(&self, address: &str) -> Result<Option<Uuid>, DbError> {
        let mut conn = self.db.get_conn()?;

        let owner_id = user_wallet::table
            .filter(user_wallet::address.eq(address))
            .select(user_wallet::user_id)
            .first::<Uuid>(&mut conn)
            .optional()?;

        Ok(owner_id)
    }
    fn get_all_wallets_by_user(
        &self,
        current_user_id: Uuid,
    ) -> Result<Vec<WalletWithTickerDb>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = user_wallet::table
            .inner_join(currency::table)
            .filter(user_wallet::user_id.eq(current_user_id))
            .select((
                user_wallet::id,
                user_wallet::address,
                user_wallet::balance,
                currency::ticker,
            ))
            .load::<WalletWithTickerDb>(&mut conn)?;

        Ok(result)
    }
}
