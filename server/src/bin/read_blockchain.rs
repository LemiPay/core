use server::domain::treasury::CurrencyAddress;
use server::infrastructure::blockchain::BlockchainService;
use server::infrastructure::blockchain::ethereum_service::EthereumService;

async fn test_read_blockchain() {
    let service = EthereumService::new();

    /// Example: get_supported_tokens function with a known currency address.
    // let x = service.get_supported_tokens(
    //     CurrencyAddress::new("0x1c7d4b196cb0c7b01d743fbc6116a902379c7238".to_string())
    //         .expect("Invalid currency address"),
    // ).await.expect("Failed to get supported tokens");
    //
    // println!("{}", x.enabled);
    // println!("{:?}", x.lemipayCurrencyId);

    /// Example: Get logs
    let x = service
        .get_events(0, 1)
        .await
        .expect("Failed to get events");

    println!("Number of events: {}", x.len());
    // for log in x {
    //     println!("Event! token: {:?} currency: {:?}", log.token, log.currency_id);
    // }

    // // Instantiate the contract instance.
    // let weth = address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    // let erc20 = ERC20::new(weth, provider);
    //
    // // Fetch the balance of WETH for a given address.
    // let owner = address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    // let balance = erc20.balanceOf(owner).call().await?;
    //
    // println!("WETH Balance of {owner}: {balance}");

    // Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test_read_blockchain());
}
