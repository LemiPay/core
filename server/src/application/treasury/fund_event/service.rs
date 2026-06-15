use std::sync::Arc;

use crate::application::treasury::fund_event::errors::ProcessingError;
use crate::application::treasury::traits::fund_event_repo::FundEventRepository;
use crate::infrastructure::blockchain::events::FundData;

pub struct FundEventService {
    repo: Arc<dyn FundEventRepository>,
}

impl FundEventService {
    pub fn new(repo: Arc<dyn FundEventRepository>) -> Self {
        Self { repo }
    }

    pub fn process_events(
        &self,
        events: &[FundData],
        last_processed_block: u64,
    ) -> Result<(), ProcessingError> {
        if events.is_empty() {
            return Ok(());
        }

        self.repo.process_events(events, last_processed_block)?;

        Ok(())
    }

    pub fn update_sync_state(&self, last_processed_block: u64) -> Result<(), ProcessingError> {
        self.repo.update_sync_state(last_processed_block)?;
        Ok(())
    }
}
