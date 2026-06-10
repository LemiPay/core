use std::sync::Arc;

use crate::application::treasury::traits::fund_event_repo::FundEventRepository;
use crate::infrastructure::blockchain::{BlockchainService, ContractEvent, FundData};

const END_BLOCK_GAP: u64 = 64;
const BATCH_SIZE: u64 = 10;
const UPDATE_BLOCK_SIZE: u64 = 100; // 700 in prod.

#[derive(Clone)]
pub struct BlockchainSyncService {
    pub repo: Arc<dyn FundEventRepository>,
    pub blockchain_service: Arc<dyn BlockchainService>,
}

impl BlockchainSyncService {
    pub async fn cold_start_sync(&self) {
        let start_block = self
            .repo
            .get_last_processed_block()
            .expect("Failed to read last_processed_block from DB");

        let latest_block = self
            .blockchain_service
            .get_block_number()
            .await
            .expect("Failed to get latest block number")
            - END_BLOCK_GAP;

        if start_block >= latest_block {
            println!("Already up to date at block {}", start_block);
            return;
        }

        let total_blocks = latest_block - start_block;
        let mut processed: u64 = 0;
        let mut fund_events: Vec<FundData> = Vec::new();
        let mut blocks_since_flush: u64 = 0;

        println!("latest_block={latest_block}");
        println!("start_block={}", start_block + 1);
        println!("batch_size={BATCH_SIZE}");
        println!("update_block_size={UPDATE_BLOCK_SIZE}");
        println!("total_blocks_to_process={total_blocks}");
        println!();

        let mut current = start_block + 1;

        while current <= latest_block {
            let to = std::cmp::min(current + BATCH_SIZE - 1, latest_block);
            let batch_len = to - current + 1;

            let mut event_count = 0usize;

            match self.blockchain_service.get_events(current, to).await {
                Ok(events) => {
                    event_count = events.len();
                    for event in events {
                        if let ContractEvent::Fund(data) = event {
                            fund_events.push(data);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("RPC FAILED [{current}, {to}]: {e}");
                }
            }

            blocks_since_flush += batch_len;
            processed += batch_len;

            let overall_pct = (processed as f64 / total_blocks as f64) * 100.0;
            let flush_pct = (blocks_since_flush as f64 / UPDATE_BLOCK_SIZE as f64) * 100.0;
            let flush_remaining = UPDATE_BLOCK_SIZE.saturating_sub(blocks_since_flush);

            println!(
                "[{current}, {to}] events={event_count} | next flush in {flush_remaining} ({flush_pct:.0}%) | overall: {processed}/{total_blocks} ({overall_pct:.1}%)"
            );

            if blocks_since_flush >= UPDATE_BLOCK_SIZE || to == latest_block {
                let flush_block = to;
                if fund_events.is_empty() {
                    self.repo
                        .update_sync_state(flush_block)
                        .expect("Failed to update sync state");
                    println!(
                        ">>> Checkpoint advanced to block {} (no events)",
                        flush_block
                    );
                } else {
                    let count = fund_events.len();
                    self.repo
                        .process_events(&fund_events, flush_block)
                        .expect("Failed to process events");
                    println!(
                        ">>> Flushed {} events, sync at block {}",
                        count, flush_block
                    );
                    fund_events = Vec::new();
                }
                blocks_since_flush = 0;
            }

            current = to + 1;
        }
    }
}
