use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub address: String,
    pub currency_ticker: String,
}

#[derive(Deserialize)]
pub struct FaucetAmountRequest {
    pub amount: String,
}

#[derive(Deserialize)]
pub struct TransferRequest {
    pub sender_wallet_id: Uuid,
    pub receiver_address: String,
    pub amount: String,
}

#[derive(Deserialize)]
pub struct CurrencyQuery {
    pub currency: String,
}

#[derive(Serialize)]
pub struct UserWalletResponse {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct WalletWithTickerResponse {
    pub wallet_id: Uuid,
    pub address: String,
    pub balance: BigDecimal,
    pub currency_id: Uuid,
    pub ticker: String,
}

#[derive(Serialize)]
pub struct AddressGroupResponse {
    pub address: String,
    pub currencies: Vec<WalletWithTickerResponse>,
}
