use alloy::sol;

sol!(
    #[sol(rpc)]
    LemiPayVault,
    "src/infrastructure/blockchain/contracts/abi/LemiPayVault.json"
);
