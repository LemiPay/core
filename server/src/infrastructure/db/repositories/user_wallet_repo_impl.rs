use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::{UserWalletDetails, UserWalletWithTickerDetails};
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::UserId;
use crate::infrastructure::db::{
    models::treasury::{NewUserWalletModel, UserWalletModel},
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselUserWalletRepository {
    db: DbPool,
}

impl DieselUserWalletRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

fn model_to_domain(model: UserWalletModel) -> UserWallet {
    UserWallet::rehydrate(
        UserWalletId(model.id),
        model.address,
        UserId(model.user_id),
        Money {
            amount: model.balance,
            currency: CurrencyId(model.currency_id),
        },
    )
}

fn model_to_details(model: UserWalletModel) -> UserWalletDetails {
    UserWalletDetails {
        id: model.id,
        address: model.address,
        user_id: model.user_id,
        currency_id: model.currency_id,
        balance: model.balance,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

impl UserWalletRepository for DieselUserWalletRepository {
    fn save(&self, wallet: &UserWallet) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_wallet = NewUserWalletModel {
            id: wallet.id.0,
            address: wallet.address.clone(),
            user_id: wallet.user_id.0,
            currency_id: wallet.balance.currency.0,
            balance: wallet.balance.amount.clone(),
        };

        diesel::insert_into(schema::user_wallet::table)
            .values(&new_wallet)
            .on_conflict(schema::user_wallet::id)
            .do_update()
            .set((
                schema::user_wallet::balance.eq(&wallet.balance.amount),
                schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }

    fn find_by_id(&self, id: UserWalletId) -> Result<Option<UserWallet>, RepoError> {
        let mut conn = self.get_conn()?;

        let model = schema::user_wallet::table
            .filter(schema::user_wallet::id.eq(id.0))
            .select(UserWalletModel::as_select())
            .first::<UserWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(model.map(model_to_domain))
    }

    fn find_by_address_and_currency(
        &self,
        address: &str,
        currency: CurrencyId,
    ) -> Result<Option<UserWallet>, RepoError> {
        let mut conn = self.get_conn()?;

        let model = schema::user_wallet::table
            .filter(schema::user_wallet::address.eq(address))
            .filter(schema::user_wallet::currency_id.eq(currency.0))
            .select(UserWalletModel::as_select())
            .first::<UserWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(model.map(model_to_domain))
    }

    fn find_owner_of_address(&self, address: &str) -> Result<Option<UserId>, RepoError> {
        let mut conn = self.get_conn()?;

        let owner = schema::user_wallet::table
            .filter(schema::user_wallet::address.eq(address))
            .select(schema::user_wallet::user_id)
            .first::<Uuid>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(owner.map(UserId))
    }

    fn transfer(
        &self,
        sender: UserWalletId,
        receiver: UserWalletId,
        amount: &Money,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        let amount_value = amount.amount.clone();
        let currency_id = amount.currency.0;

        conn.transaction::<(), diesel::result::Error, _>(|tx_conn| {
            let debited = diesel::update(
                schema::user_wallet::table
                    .filter(schema::user_wallet::id.eq(sender.0))
                    .filter(schema::user_wallet::currency_id.eq(currency_id))
                    .filter(schema::user_wallet::balance.ge(amount_value.clone())),
            )
            .set((
                schema::user_wallet::balance
                    .eq(schema::user_wallet::balance - amount_value.clone()),
                schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx_conn)?;

            if debited != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            let credited = diesel::update(
                schema::user_wallet::table
                    .filter(schema::user_wallet::id.eq(receiver.0))
                    .filter(schema::user_wallet::currency_id.eq(currency_id)),
            )
            .set((
                schema::user_wallet::balance
                    .eq(schema::user_wallet::balance + amount_value.clone()),
                schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx_conn)?;

            if credited != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
        .map_err(|_| RepoError::Insert)
    }

    fn get_details(&self, id: UserWalletId) -> Result<Option<UserWalletDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let model = schema::user_wallet::table
            .filter(schema::user_wallet::id.eq(id.0))
            .select(UserWalletModel::as_select())
            .first::<UserWalletModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(model.map(model_to_details))
    }

    fn list_with_ticker_by_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserWalletWithTickerDetails>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::user_wallet::table
            .inner_join(schema::currency::table)
            .filter(schema::user_wallet::user_id.eq(user_id.0))
            .select((
                schema::user_wallet::id,
                schema::user_wallet::address,
                schema::user_wallet::balance,
                schema::currency::currency_id,
                schema::currency::ticker,
            ))
            .load::<(Uuid, String, BigDecimal, Uuid, String)>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(
                |(wallet_id, address, balance, currency_id, ticker)| UserWalletWithTickerDetails {
                    wallet_id,
                    address,
                    balance,
                    currency_id,
                    ticker,
                },
            )
            .collect::<Vec<_>>())
    }
}
