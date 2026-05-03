use axum::{
    Json,
    extract::{Path, State},
};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::treasury::UserWalletId;
use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::dto::{FaucetAmountRequest, UserWalletResponse},
};
use crate::setup::state::SharedState;

pub async fn faucet_fund_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(wallet_id): Path<Uuid>,
    Json(req): Json<FaucetAmountRequest>,
) -> Result<Json<UserWalletResponse>, AppError> {
    let amount = BigDecimal::from_str(&req.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;

    let details = state
        .treasury_service
        .faucet_fund_wallet
        .execute(user.user_id, UserWalletId(wallet_id), amount)
        .map_err(AppError::from)?;

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
