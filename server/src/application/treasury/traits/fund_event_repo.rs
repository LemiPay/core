use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::BlockchainEventDetails;
use crate::infrastructure::blockchain::events::{FundData, WithdrawData};

pub trait FundEventRepository: Send + Sync {
    /// Atomically persists a batch of Fund events: inserts blockchain_event rows,
    /// credits the corresponding user_wallet balances, and updates the sync state.
    /// Returns `RepoError::Query` if a currency or wallet lookup fails.
    fn process_events(
        &self,
        events: &[FundData],
        last_processed_block: u64,
    ) -> Result<(), RepoError>;

    /// Atomically persists a batch of Withdraw events: inserts blockchain_event rows,
    /// debits the corresponding user_wallet balances, and updates the sync state.
    fn process_withdraw_events(
        &self,
        events: &[WithdrawData],
        last_processed_block: u64,
    ) -> Result<(), RepoError>;

    /// Inserts a single blockchain event (e.g. a Withdraw initiated via the API).
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
    ) -> Result<(), RepoError>;

    /// Updates only the sync state checkpoint without processing any events.
    fn update_sync_state(&self, last_processed_block: u64) -> Result<(), RepoError>;

    /// Returns the last processed block from the sync state, or 0 if not found.
    fn get_last_processed_block(&self) -> Result<u64, RepoError>;

    /// Returns blockchain events for the given wallet addresses, ordered by most recent first.
    fn list_by_wallet_addresses(
        &self,
        wallet_addresses: &[String],
    ) -> Result<Vec<BlockchainEventDetails>, RepoError>;
}
