use server::infrastructure::blockchain::ethereum_service::EthereumService;
use server::infrastructure::blockchain::{BlockchainService, ContractEvent, FundData};

const START_BLOCK: u64 = 11023370;
const END_BLOCK_GAP: u64 = 64;
const BATCH_SIZE: u64 = 10;
const UPDATE_BLOCK_SIZE: u64 = 100; // 700 in prod.

#[tokio::main]
async fn main() {
    let service = EthereumService::new();

    let latest_block = service
        .get_block_number()
        .await
        .expect("Failed to get latest block number")
        - END_BLOCK_GAP;

    let total_blocks = latest_block.saturating_sub(START_BLOCK).saturating_add(1);
    let mut processed: u64 = 0;
    let mut fund_events: Vec<FundData> = Vec::new();
    let mut blocks_since_flush: u64 = 0;

    println!("latest_block={latest_block}");
    println!("start_block={START_BLOCK}");
    println!("batch_size={BATCH_SIZE}");
    println!("update_block_size={UPDATE_BLOCK_SIZE}");
    println!("total_blocks_to_process={total_blocks}");
    println!();

    let mut current = START_BLOCK;

    while current <= latest_block {
        let to = std::cmp::min(current + BATCH_SIZE - 1, latest_block);
        let batch_len = to - current + 1;

        let mut event_count = 0usize;

        match service.get_events(current, to).await {
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
            flush_fund_events(std::mem::take(&mut fund_events));
            blocks_since_flush = 0;
        }

        current = to + 1;
    }
}

fn flush_fund_events(events: Vec<FundData>) {
    if events.is_empty() {
        return;
    }
    println!(">>> FLUSH: processing {} Fund events...", events.len());
    for event in &events {
        println!(
            "  Fund {{ sender: {:?}, wallet: {:?}, token: {:?}, gross: {} }}",
            event.sender, event.wallet_address, event.token, event.gross_amount
        );
    }
    println!(">>> FLUSH complete");
}
