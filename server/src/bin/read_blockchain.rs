use server::infrastructure::blockchain::ethereum_service::EthereumService;

async fn test_read_blockchain() {
    let _ = EthereumService::new();

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
