use server::infrastructure::blockchain::BlockchainService;
use server::infrastructure::blockchain::ethereum_service::EthereumService;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let blockchain_service = Arc::new(EthereumService::new());

    let block_number = blockchain_service
        .get_block_number()
        .await
        .expect("Failed to get latest block number");

    println!("Latest block number: {block_number}");
}
