use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Blockchain service error")]
    BlockchainService,
}
