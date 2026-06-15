use std::sync::Arc;
use tokio::time::{Duration, interval};

use crate::application::startup::config::{
    BATCH_SIZE, END_BLOCK_GAP, POLL_INTERVAL_SECS, UPDATE_BLOCK_SIZE,
};
use crate::application::treasury::traits::fund_event_repo::FundEventRepository;
use crate::infrastructure::blockchain::{BlockchainService, ContractEvent, FundData, WithdrawData};

pub struct LiveSyncService {
    repo: Arc<dyn FundEventRepository>,
    blockchain_service: Arc<dyn BlockchainService>,
}

impl LiveSyncService {
    pub fn new(
        repo: Arc<dyn FundEventRepository>,
        blockchain_service: Arc<dyn BlockchainService>,
    ) -> Self {
        Self {
            repo,
            blockchain_service,
        }
    }

    pub async fn start(&self) {
        let mut timer = interval(Duration::from_secs(POLL_INTERVAL_SECS));
        loop {
            timer.tick().await;

            let last_processed = match self.repo.get_last_processed_block() {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("[LiveSync] Failed to read last_processed_block: {e:?}");
                    continue;
                }
            };

            let latest = match self.blockchain_service.get_block_number().await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("[LiveSync] Failed to get block number: {e}");
                    continue;
                }
            };

            let confirmed = latest.saturating_sub(END_BLOCK_GAP);
            if confirmed <= last_processed {
                continue;
            }

            let total_blocks = confirmed - last_processed;
            let mut processed: u64 = 0;
            let mut current = last_processed + 1;
            let mut fund_events: Vec<FundData> = Vec::new();
            let mut withdraw_events: Vec<WithdrawData> = Vec::new();
            let mut blocks_since_flush: u64 = 0;

            while current <= confirmed {
                let to = std::cmp::min(current + BATCH_SIZE - 1, confirmed);
                let batch_len = to - current + 1;

                match self.blockchain_service.get_events(current, to).await {
                    Ok(events) => {
                        for event in events {
                            match event {
                                ContractEvent::Fund(data) => fund_events.push(data),
                                ContractEvent::Withdraw(data) => withdraw_events.push(data),
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[LiveSync] RPC FAILED [{current}, {to}]: {e}");
                    }
                }

                processed += batch_len;
                blocks_since_flush += batch_len;
                let remaining_pct =
                    ((total_blocks - processed) as f64 / total_blocks as f64) * 100.0;

                if total_blocks > BATCH_SIZE {
                    println!(
                        "[LiveSync] [{current}, {to}] last_block={to} | remaining {:.1}%",
                        remaining_pct
                    );
                }

                if blocks_since_flush >= UPDATE_BLOCK_SIZE || to == confirmed {
                    let flush_block = to;
                    let has_fund = !fund_events.is_empty();
                    let has_withdraw = !withdraw_events.is_empty();

                    if !has_fund && !has_withdraw {
                        if let Err(e) = self.repo.update_sync_state(flush_block) {
                            eprintln!(
                                "[LiveSync] Failed to update sync state to block {}: {e:?}",
                                flush_block
                            );
                        }
                    } else {
                        if has_fund {
                            match self.repo.process_events(&fund_events, flush_block) {
                                Ok(()) => {
                                    println!(
                                        "[LiveSync] Flushed {} Fund events, sync at block {}",
                                        fund_events.len(),
                                        flush_block
                                    );
                                }
                                Err(e) => {
                                    eprintln!(
                                        "[LiveSync] Failed to process Fund events up to block {}: {e:?}",
                                        flush_block
                                    );
                                }
                            }
                            fund_events = Vec::new();
                        }
                        if has_withdraw {
                            match self
                                .repo
                                .process_withdraw_events(&withdraw_events, flush_block)
                            {
                                Ok(()) => {
                                    println!(
                                        "[LiveSync] Flushed {} Withdraw events, sync at block {}",
                                        withdraw_events.len(),
                                        flush_block
                                    );
                                }
                                Err(e) => {
                                    eprintln!(
                                        "[LiveSync] Failed to process Withdraw events up to block {}: {e:?}",
                                        flush_block
                                    );
                                }
                            }
                            withdraw_events = Vec::new();
                        }
                    }
                    blocks_since_flush = 0;
                }

                current = to + 1;
            }
        }
    }
}
