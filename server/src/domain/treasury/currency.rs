use alloy::primitives::Address;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

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
