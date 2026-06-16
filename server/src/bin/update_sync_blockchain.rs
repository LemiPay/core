use server::application::startup::config::UPDATE_BLOCK_SIZE;
use server::application::treasury::traits::fund_event_repo::FundEventRepository;
use server::infrastructure::blockchain::BlockchainService;
use server::infrastructure::blockchain::ethereum_service::EthereumService;
use server::infrastructure::db::pool::create_pool;
use server::infrastructure::db::repositories::fund_event_repo_impl::DieselFundEventRepository;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    dotenvy::from_filename("../.env").ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let pool = create_pool(&database_url);
    let repo = DieselFundEventRepository::new(pool);
    let blockchain_service = EthereumService::new();

    let previous = repo
        .get_last_processed_block()
        .expect("Failed to read last_processed_block from DB");

    let latest_block = blockchain_service
        .get_block_number()
        .await
        .expect("Failed to get latest Sepolia block number");

    let target_block = latest_block.saturating_sub(UPDATE_BLOCK_SIZE);

    repo.update_sync_state(target_block)
        .expect("Failed to update blockchain_sync_state");

    println!("Latest Sepolia block: {latest_block}");
    println!("Target block (latest - {UPDATE_BLOCK_SIZE}): {target_block}");
    println!("Previous last_processed_block: {previous}");
    println!("Updated blockchain_sync_state (lemipay_vault) -> {target_block}");
}
