use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Blockchain service error: {0}")]
    BlockchainService(String),
}
