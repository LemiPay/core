use std::str::FromStr;

use alloy::primitives::{Address, B256, U256};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::BlockchainEventDetails;
use crate::application::treasury::traits::fund_event_repo::FundEventRepository;
use crate::infrastructure::blockchain::events::{FundData, WithdrawData};
use crate::infrastructure::db::{
    models::treasury::{BlockchainEventModel, NewBlockchainEventModel},
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

fn b256_to_uuid(val: &B256) -> Uuid {
    let bytes = val.as_slice();
    Uuid::from_slice(&bytes[..16]).expect("valid UUID bytes")
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
    fn insert_event(
        &self,
        event_type: &str,
        sender: &str,
        wallet_address: &str,
        token_address: &str,
        currency_id: Uuid,
        gross_amount: BigDecimal,
        fee_amount: BigDecimal,
        net_amount: BigDecimal,
        tx_hash: &str,
        block_number: i64,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_event = NewBlockchainEventModel {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            sender: sender.to_string(),
            wallet_address: wallet_address.to_string(),
            token_address: token_address.to_string(),
            currency_id,
            gross_amount,
            fee_amount,
            net_amount,
            tx_hash: tx_hash.to_string(),
            block_number,
        };

        diesel::insert_into(schema::blockchain_event::table)
            .values(&new_event)
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }

    fn process_withdraw_events(
        &self,
        events: &[WithdrawData],
        last_processed_block: u64,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        conn.transaction::<(), diesel::result::Error, _>(|tx_conn| {
            for event in events {
                let wallet_addr = extract_address_from_b256(&event.wallet_address);
                let receiver = address_to_str(&event.receiver);
                let token_addr = address_to_str(&event.token);
                let currency_uuid = b256_to_uuid(&event.currency_id);

                let decimals: i16 = schema::currency::table
                    .filter(schema::currency::currency_id.eq(currency_uuid))
                    .select(schema::currency::decimals)
                    .first::<i16>(tx_conn)
                    .map_err(|e| {
                        match e {
                            diesel::result::Error::NotFound => {
                                println!("Database error: Currency not found for currency_id: {currency_uuid}");
                            },
                            _ => panic!("Database error: {e}"),
                        }
                    })
                    .map_err(|_| diesel::result::Error::NotFound)?;

                let wallet_uuid: Uuid = schema::user_wallet::table
                    .filter(schema::user_wallet::address.eq(&wallet_addr.to_lowercase()))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .select(schema::user_wallet::id)
                    .first::<Uuid>(tx_conn)
                    .map_err(|e| {
                        match e {
                            diesel::result::Error::NotFound => {
                                println!("Database error: User wallet not found for address: {wallet_addr} and currency_id: {currency_uuid}");
                            },
                            _ => panic!("Database error: {e}"),
                        }
                    })
                    .map_err(|_| diesel::result::Error::NotFound)?;

                let divisor = BigDecimal::from(10u64.pow(decimals as u32));

                let net = b256_to_bigdecimal(&event.net_amount) / &divisor;

                diesel::update(schema::user_wallet::table)
                    .filter(schema::user_wallet::id.eq(wallet_uuid))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .set((
                        schema::user_wallet::balance.eq(schema::user_wallet::balance - &net),
                        schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(tx_conn)?;

                let gross = b256_to_bigdecimal(&event.gross_amount) / &divisor;
                let fee = b256_to_bigdecimal(&event.fee_amount) / &divisor;

                let new_event = NewBlockchainEventModel {
                    id: Uuid::new_v4(),
                    event_type: "Withdraw".to_string(),
                    sender: receiver,
                    wallet_address: wallet_addr,
                    token_address: token_addr,
                    currency_id: currency_uuid,
                    gross_amount: gross,
                    fee_amount: fee,
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
                let currency_uuid = b256_to_uuid(&event.currency_id);

                let decimals: i16 = schema::currency::table
                    .filter(schema::currency::currency_id.eq(currency_uuid))
                    .select(schema::currency::decimals)
                    .first::<i16>(tx_conn)
                    .map_err(|e| {
                        match e {
                            diesel::result::Error::NotFound => {
                                println!("Database error: Currency not found for currency_id: {currency_uuid}");
                            },
                            _ => panic!("Database error: {e}"),
                        }
                    })
                    .map_err(|_| diesel::result::Error::NotFound)?;

                let wallet_uuid: Uuid = schema::user_wallet::table
                    .filter(schema::user_wallet::address.eq(&wallet_addr.to_lowercase()))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .select(schema::user_wallet::id)
                    .first::<Uuid>(tx_conn)
                    .map_err(|e| {
                        match e {
                            diesel::result::Error::NotFound => {
                                println!("Database error: User wallet not found for address: {wallet_addr} and currency_id: {currency_uuid}");
                            },
                            _ => panic!("Database error: {e}"),
                        }
                    })
                    .map_err(|_| diesel::result::Error::NotFound)?;

                let divisor = BigDecimal::from(10u64.pow(decimals as u32));

                let net = b256_to_bigdecimal(&event.net_amount) / &divisor;

                diesel::update(schema::user_wallet::table)
                    .filter(schema::user_wallet::id.eq(wallet_uuid))
                    .filter(schema::user_wallet::currency_id.eq(currency_uuid))
                    .set((
                        schema::user_wallet::balance.eq(schema::user_wallet::balance + &net),
                        schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .execute(tx_conn)?;

                let gross = b256_to_bigdecimal(&event.gross_amount) / &divisor;
                let fee = b256_to_bigdecimal(&event.fee_amount) / &divisor;

                let new_event = NewBlockchainEventModel {
                    id: Uuid::new_v4(),
                    event_type: "Fund".to_string(),
                    sender,
                    wallet_address: wallet_addr,
                    token_address: token_addr,
                    currency_id: currency_uuid,
                    gross_amount: gross,
                    fee_amount: fee,
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

    fn list_by_wallet_addresses(
        &self,
        wallet_addresses: &[String],
    ) -> Result<Vec<BlockchainEventDetails>, RepoError> {
        use schema::blockchain_event::dsl::*;

        let mut conn = self.get_conn()?;

        let lowercased: Vec<String> = wallet_addresses.iter().map(|a| a.to_lowercase()).collect();

        let models: Vec<BlockchainEventModel> = blockchain_event
            .filter(wallet_address.eq_any(&lowercased))
            .order(created_at.desc())
            .load::<BlockchainEventModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(models
            .into_iter()
            .map(|m| BlockchainEventDetails {
                id: m.id,
                event_type: m.event_type,
                sender: m.sender,
                wallet_address: m.wallet_address,
                token_address: m.token_address,
                currency_id: m.currency_id,
                gross_amount: m.gross_amount,
                fee_amount: m.fee_amount,
                net_amount: m.net_amount,
                tx_hash: m.tx_hash,
                block_number: m.block_number,
                created_at: m.created_at,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_b256_to_uuid_right_padded() {
        // Contract storage format: UUID in first 16 bytes, zeros in last 16
        let hex = "0x33de6c7c62a24182813a9005183be70d00000000000000000000000000000000";
        let b256 = B256::from_str(hex).unwrap();
        let uuid = b256_to_uuid(&b256);
        assert_eq!(uuid.to_string(), "33de6c7c-62a2-4182-813a-9005183be70d");
    }

    #[test]
    fn test_b256_to_uuid_event_data() {
        // Actual event currency_id from the log
        let hex = "0x7e24923ada535d33a4774e2fe5b6de78af1ab6fc962c0722668f2b75f4a92e67";
        let b256 = B256::from_str(hex).unwrap();
        let uuid = b256_to_uuid(&b256);
        assert_eq!(uuid.to_string(), "7e24923a-da53-5d33-a477-4e2fe5b6de78");
    }

    #[test]
    fn test_b256_to_uuid_left_padded_as_uint256() {
        // UUID stored as uint256 (left-padded with 16 zero bytes)
        let hex = "0x0000000000000000000000000000000033de6c7c62a24182813a9005183be70d";
        let b256 = B256::from_str(hex).unwrap();
        let uuid = b256_to_uuid(&b256);
        // bytes[..16] are all zeros
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn test_b256_to_uuid_last_16_bytes() {
        // UUID stored in last 16 bytes (uint128 right-aligned)
        let hex = "0x0000000000000000000000000000000033de6c7c62a24182813a9005183be70d";
        let b256 = B256::from_str(hex).unwrap();
        let bytes = b256.as_slice();
        let uuid = Uuid::from_slice(&bytes[16..]).unwrap();
        assert_eq!(uuid.to_string(), "33de6c7c-62a2-4182-813a-9005183be70d");
    }

    #[test]
    fn test_extract_address_from_wallet_address() {
        // wallet_address: left-padded address
        let hex = "0x00000000000000000000000026583527b405434313ec0a88f629fb99b42e1e6d";
        let b256 = B256::from_str(hex).unwrap();
        let addr = extract_address_from_b256(&b256);
        assert_eq!(addr, "0x26583527b405434313ec0a88f629fb99b42e1e6d");
    }

    #[test]
    fn test_address_to_str() {
        let addr = Address::from_str("0x1c7d4b196cb0c7b01d743fbc6116a902379c7238").unwrap();
        assert_eq!(
            address_to_str(&addr),
            "0x1c7d4b196cb0c7b01d743fbc6116a902379c7238"
        );
    }

    #[test]
    fn test_b256_to_bigdecimal() {
        let val = U256::from_str("1000000").unwrap();
        let dec = b256_to_bigdecimal(&val);
        assert_eq!(dec, BigDecimal::from_str("1000000").unwrap());
    }

    #[test]
    fn test_scale_by_decimals() {
        // USDC: 999000 raw / 10^6 = 0.999
        let raw = BigDecimal::from_str("999000").unwrap();
        let divisor = BigDecimal::from(10u64.pow(6));
        let scaled = raw / divisor;
        assert_eq!(scaled.to_string(), "0.999");
        assert_eq!(scaled, BigDecimal::from_str("0.999").unwrap());
    }

    #[test]
    fn test_b256_to_str() {
        let hex = "0x75a2bb9b5c75fa96c331b2dd1a05565329a448c54f934f6ca0e5b8f55b42185b";
        let b256 = B256::from_str(hex).unwrap();
        assert_eq!(b256_to_str(&b256), format!("{:?}", b256));
    }
}
