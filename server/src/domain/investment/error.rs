#[derive(Debug)]
pub enum InvestmentError {
    InvalidAmount,
    InvalidStatusTransition,
    NotMatured,
    AlreadyWithdrawn,
    StrategyNotFound,
}
