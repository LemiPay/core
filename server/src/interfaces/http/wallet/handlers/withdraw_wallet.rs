use alloy::primitives::{Address, B256, U256, eip191_hash_message};
use axum::{
    Json,
    extract::{Path, State},
};
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::treasury::UserWalletId;
use crate::infrastructure::blockchain::error::BlockchainError;
use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::{
        dto::{UserWalletResponse, WithdrawRequest},
        withdraw_message::build_withdraw_authorization_message,
    },
};
use crate::setup::state::SharedState;

fn addr_to_b256(addr: &str) -> Result<B256, AppError> {
    let parsed: Address = addr
        .parse()
        .map_err(|_| AppError::BadRequest("Dirección inválida".into()))?;
    let mut bytes = [0u8; 32];
    bytes[12..].copy_from_slice(parsed.as_slice());
    Ok(B256::from(bytes))
}

pub fn decimal_to_u256(amount: &BigDecimal, decimals: u8) -> Result<U256, AppError> {
    let multiplier = BigUint::from(10u8).pow(decimals as u32);

    let scaled: BigDecimal = amount * multiplier;

    let integer: BigDecimal = scaled.with_scale(0);

    if integer < BigDecimal::from(0) {
        return Err(AppError::InvalidAmount("Amount cannot be negative".into()));
    }

    U256::from_str(&integer.to_string())
        .map_err(|_| AppError::InvalidAmount("Amount does not fit into U256".into()))
}

pub async fn withdraw_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(wallet_id): Path<Uuid>,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<UserWalletResponse>, AppError> {
    let amount = BigDecimal::from_str(&req.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;

    let addr: Address = req
        .address
        .trim()
        .parse()
        .map_err(|_| AppError::BadRequest("Dirección inválida".into()))?;

    let expected_message =
        build_withdraw_authorization_message(wallet_id, req.amount.trim(), &addr, req.uri.trim());

    if req.message != expected_message {
        return Err(AppError::BadRequest(
            "El mensaje firmado no coincide con el retiro solicitado.".into(),
        ));
    }

    let signed_message = eip191_hash_message(req.message.as_bytes());

    let sig: alloy::primitives::Bytes = req
        .signature
        .parse()
        .map_err(|_| AppError::BadRequest("Firma inválida".into()))?;

    let is_valid = state
        .blockchain_service
        .verify_signature(sig, addr, signed_message)
        .await;

    if !is_valid {
        return Err(AppError::Forbidden("Firma inválida".into()));
    }

    let details = state
        .treasury_service
        .withdraw_wallet
        .execute(user.user_id, UserWalletId(wallet_id), amount.clone())
        .map_err(AppError::from)?;

    let currency_info = state
        .currency_repo
        .find_by_id(crate::domain::treasury::CurrencyId(details.currency_id))
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::BadRequest("Moneda no encontrada".into()))?;

    let token_addr: Address = currency_info
        .token_address
        .parse()
        .map_err(|_| AppError::Internal)?;

    let wallet_b256 = addr_to_b256(&details.address)?;
    let raw_amount = decimal_to_u256(&amount, currency_info.decimals as u8)?;

    let tx_hash = state
        .blockchain_service
        .withdraw(addr, wallet_b256, token_addr, raw_amount)
        .await
        .map_err(|e: BlockchainError| {
            AppError::BadRequest(format!("Error al ejecutar el retiro en blockchain: {e}"))
        })?;

    state
        .fund_event_repo
        .insert_event(
            "Withdraw",
            &req.address,
            &details.address,
            &currency_info.token_address,
            details.currency_id,
            amount.clone(),
            BigDecimal::zero(),
            amount,
            &tx_hash,
            0,
        )
        .map_err(|_| AppError::Internal)?;

    Ok(Json(UserWalletResponse {
        id: details.id,
        address: details.address,
        user_id: details.user_id,
        currency_id: details.currency_id,
        balance: details.balance,
        created_at: details.created_at,
        updated_at: details.updated_at,
    }))
}
