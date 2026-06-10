use crate::application::common::repo_error::RepoError;
use crate::infrastructure::blockchain::events::FundData;

pub trait FundEventRepository: Send + Sync {
    /// Atomically persists a batch of Fund events: inserts blockchain_event rows,
    /// credits the corresponding user_wallet balances, and updates the sync state.
    /// Returns `RepoError::Query` if a currency or wallet lookup fails.
    fn process_events(
        &self,
        events: &[FundData],
        last_processed_block: u64,
    ) -> Result<(), RepoError>;

    /// Updates only the sync state checkpoint without processing any events.
    fn update_sync_state(&self, last_processed_block: u64) -> Result<(), RepoError>;

    /// Returns the last processed block from the sync state, or 0 if not found.
    fn get_last_processed_block(&self) -> Result<u64, RepoError>;
}
