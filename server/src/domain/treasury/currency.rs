use alloy::primitives::Address;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Blockchain {
    Ethereum,
    Sepolia,
    Arbitrum,
    Base,
    Polygon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    pub id: CurrencyId,
    pub name: String,
    pub ticker: String,
    pub blockchain: Blockchain,
    pub token_address: CurrencyAddress,
    pub token_currency_id: Option<String>,
    pub decimals: i16,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurrencyId(pub Uuid);

impl CurrencyId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for CurrencyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurrencyAddress(pub Address);

impl CurrencyAddress {
    pub fn new(addr: String) -> Result<Self, String> {
        match addr.parse() {
            Ok(address) => Ok(Self(address)),
            Err(_) => Err("Invalid address!".to_string()),
        }
    }

    pub fn as_address(&self) -> &Address {
        &self.0
    }
}
