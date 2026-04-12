use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user_wallet::{UserWallet, WalletWithTickerDb};
use std::str::FromStr;

use crate::security::auth_extractor::AuthUser;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct NewWalletRequest {
    pub address: String,
    pub currency_ticker: String,
}
#[derive(Deserialize, Serialize)]
pub struct FaucetFundRequest {
    pub amount: String,
}

#[derive(Serialize)]
pub struct AddressGroup {
    pub address: String,
    pub currencies: Vec<WalletWithTickerDb>,
}

#[derive(Deserialize)]
pub struct FundTransferRequest {
    pub sender_wallet_id: Uuid,
    pub receiver_address: String,
    pub amount: String,
}

pub async fn create_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(payload): Json<NewWalletRequest>,
) -> Result<Json<UserWallet>, AppError> {
    let new_wallet = state
        .user_wallet_service
        .create_wallet(payload, user.user_id)?;
    Ok(Json(new_wallet))
}

pub async fn faucet_fund_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(wallet_id): Path<Uuid>,
    Json(payload): Json<FaucetFundRequest>,
) -> Result<Json<UserWallet>, AppError> {
    let amount_bd = BigDecimal::from_str(&payload.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;
    let result =
        state
            .user_wallet_service
            .faucet_fund_wallet(user.user_id, wallet_id, amount_bd)?;
    Ok(Json(result))
}
pub async fn faucet_withdraw_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(wallet_id): Path<Uuid>,
    Json(payload): Json<FaucetFundRequest>,
) -> Result<Json<UserWallet>, AppError> {
    let amount_bd = BigDecimal::from_str(&payload.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;
    let result =
        state
            .user_wallet_service
            .faucet_withdraw_wallet(user.user_id, wallet_id, amount_bd)?;
    Ok(Json(result))
}

pub async fn transfer_to_user_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(payload): Json<FundTransferRequest>,
) -> Result<Json<bool>, AppError> {
    let _amount_bd = BigDecimal::from_str(&payload.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;
    let result = state
        .user_wallet_service
        .transfer_funds(user.user_id, payload)?;
    Ok(Json(result))
}

pub async fn get_all_wallets(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<AddressGroup>>, AppError> {
    let result = state
        .user_wallet_service
        .get_grouped_user_wallets(user.user_id)?;
    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct CurrencyQuery {
    pub currency: String,
}
pub async fn get_my_wallet_by_address_and_ticker(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(address): Path<String>,
    Query(query): Query<CurrencyQuery>,
) -> Result<Json<UserWallet>, AppError> {
    let wallet = state
        .user_wallet_service
        .get_my_wallet_by_address_and_ticker(user.user_id, &address, query.currency)?;

    Ok(Json(wallet))
}
