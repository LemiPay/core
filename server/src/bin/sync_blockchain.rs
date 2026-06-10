use std::sync::Arc;

use server::application::startup::service::BlockchainSyncService;
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
    let repo = Arc::new(DieselFundEventRepository::new(pool));
    let blockchain_service = Arc::new(EthereumService::new());
    let sync_service = BlockchainSyncService {
        repo,
        blockchain_service,
    };

    sync_service.cold_start_sync().await;
}
