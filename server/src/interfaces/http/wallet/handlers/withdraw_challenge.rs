use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, State},
};
use std::str::FromStr;
use uuid::Uuid;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::{
        dto::{WithdrawChallengeRequest, WithdrawChallengeResponse},
        withdraw_message::build_withdraw_authorization_message,
    },
};
use crate::setup::state::SharedState;

pub async fn withdraw_wallet_challenge(
    State(_state): State<SharedState>,
    _user: AuthUser,
    Path(wallet_id): Path<Uuid>,
    Json(req): Json<WithdrawChallengeRequest>,
) -> Result<Json<WithdrawChallengeResponse>, AppError> {
    if req.amount.trim().is_empty() {
        return Err(AppError::BadRequest("Monto inválido".into()));
    }

    let _amount = bigdecimal::BigDecimal::from_str(req.amount.trim())
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;

    let address: Address = req
        .address
        .trim()
        .parse()
        .map_err(|_| AppError::BadRequest("Dirección inválida".into()))?;

    if req.uri.trim().is_empty() {
        return Err(AppError::BadRequest("URI inválida".into()));
    }

    let message = build_withdraw_authorization_message(
        wallet_id,
        req.amount.trim(),
        &address,
        req.uri.trim(),
    );

    Ok(Json(WithdrawChallengeResponse { message }))
}
