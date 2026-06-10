use std::str::FromStr;

use alloy::primitives::{Address, B256, U256};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::traits::fund_event_repo::FundEventRepository;
use crate::infrastructure::blockchain::events::FundData;
use crate::infrastructure::db::{
    models::treasury::NewBlockchainEventModel,
    pool::{DbConn, DbPool},
    schema,
};

fn address_to_str(addr: &Address) -> String {
    format!("{:?}", addr)
}

fn b256_to_str(hash: &B256) -> String {
    format!("{:?}", hash)
}

fn extract_address_from_b256(val: &B256) -> String {
    let bytes = val.as_slice();
    let addr = Address::from_slice(&bytes[12..]);
    address_to_str(&addr)
}

fn b256_to_bigdecimal(val: &U256) -> BigDecimal {
    BigDecimal::from_str(&val.to_string()).expect("U256 fits in BigDecimal")
}

pub struct DieselFundEventRepository {
    db: DbPool,
}

impl DieselFundEventRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

fn do_update_sync_state(
    tx_conn: &mut DbConn,
    last_processed_block: u64,
) -> Result<(), diesel::result::Error> {
    diesel::update(schema::blockchain_sync_state::table)
        .filter(schema::blockchain_sync_state::sync_key.eq("lemipay_vault"))
        .set((
            schema::blockchain_sync_state::last_processed_block.eq(last_processed_block as i64),
            schema::blockchain_sync_state::updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(tx_conn)?;
    Ok(())
}

impl FundEventRepository for DieselFundEventRepository {
    fn process_events(
        &self,
        events: &[FundData],
        last_processed_block: u64,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        conn.transaction::<(), diesel::result::Error, _>(|tx_conn| {
            for event in events {
                let wallet_addr = extract_address_from_b256(&event.wallet_address);
                let sender = address_to_str(&event.sender);
                let token_addr = address_to_str(&event.token);
                let currency_id_str = b256_to_str(&event.currency_id);

                let currency_uuid: Uuid = schema::currency::table
                    .filter(schema::currency::token_currency_id.eq(&currency_id_str))
                    .select(schema::currency::currency_id)
                    .first::<Uuid>(tx_conn)?;

                let wallet_uuid: Uuid = schema::user_wallet::table
                    .filter(schema::user_wallet::address.eq(&wallet_addr))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .select(schema::user_wallet::id)
                    .first::<Uuid>(tx_conn)?;

                let net = b256_to_bigdecimal(&event.net_amount);

                diesel::update(schema::user_wallet::table)
                    .filter(schema::user_wallet::id.eq(wallet_uuid))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .set((
                        schema::user_wallet::balance.eq(schema::user_wallet::balance + &net),
                        schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(tx_conn)?;

                let new_event = NewBlockchainEventModel {
                    id: Uuid::new_v4(),
                    event_type: "Fund".to_string(),
                    sender,
                    wallet_address: wallet_addr,
                    token_address: token_addr,
                    currency_id: currency_uuid,
                    gross_amount: b256_to_bigdecimal(&event.gross_amount),
                    fee_amount: b256_to_bigdecimal(&event.fee_amount),
                    net_amount: net,
                    tx_hash: b256_to_str(&event.tx_hash),
                    block_number: event.block_number as i64,
                };

                diesel::insert_into(schema::blockchain_event::table)
                    .values(&new_event)
                    .execute(tx_conn)?;
            }

            do_update_sync_state(tx_conn, last_processed_block)?;

            Ok(())
        })
        .map_err(|_| RepoError::Insert)
    }

    fn update_sync_state(&self, last_processed_block: u64) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        conn.transaction::<(), diesel::result::Error, _>(|tx_conn| {
            do_update_sync_state(tx_conn, last_processed_block)
        })
        .map_err(|_| RepoError::Insert)
    }

    fn get_last_processed_block(&self) -> Result<u64, RepoError> {
        let mut conn = self.get_conn()?;

        let block: i64 = schema::blockchain_sync_state::table
            .filter(schema::blockchain_sync_state::sync_key.eq("lemipay_vault"))
            .select(schema::blockchain_sync_state::last_processed_block)
            .first(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(block as u64)
    }
}
