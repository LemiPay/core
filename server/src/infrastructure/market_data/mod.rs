pub mod oracle;
pub mod strategies_config;
pub mod tickers;

pub use oracle::{
    AssetPriceRef, CoinGeckoPriceOracle, CompositePriceOracle, MockPriceOracle, PriceOracle,
    PriceOracleError, build_price_oracle,
};
pub use strategies_config::{StrategyDefinition, load_strategy_definitions};
pub use tickers::TickerMap;
