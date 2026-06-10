use server::infrastructure::blockchain::BlockchainService;
use server::infrastructure::blockchain::ethereum_service::EthereumService;

const START_BLOCK: u64 = 11023370;
const BATCH_SIZE: u64 = 10;

#[tokio::main]
async fn main() {
    let service = EthereumService::new();

    let latest_block = service
        .get_block_number()
        .await
        .expect("Failed to get latest block number");

    let total_blocks = latest_block.saturating_sub(START_BLOCK).saturating_add(1);
    let mut processed: u64 = 0;

    println!("latest_block={latest_block}");
    println!("start_block={START_BLOCK}");
    println!("batch_size={BATCH_SIZE}");
    println!("total_blocks_to_process={total_blocks}");
    println!();

    let mut current = START_BLOCK;

    while current <= latest_block {
        let to = std::cmp::min(current + BATCH_SIZE - 1, latest_block);
        let batch_len = to - current + 1;

        print!("[{}, {}] ... ", current, to);

        match service.get_events(current, to).await {
            Ok(events) => {
                println!("{} events", events.len());
                for event in &events {
                    event.execute_print();
                }
            }
            Err(e) => {
                eprintln!("RPC FAILED: {e}");
            }
        }

        processed += batch_len;
        let pct = (processed as f64 / total_blocks as f64) * 100.0;
        println!("      progress: {processed}/{total_blocks} ({pct:.1}%)");

        current = to + 1;
    }
}
