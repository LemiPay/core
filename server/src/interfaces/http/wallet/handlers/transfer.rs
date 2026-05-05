use axum::{Json, extract::State};
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::application::treasury::transfer_funds::dto::TransferFundsInput;
use crate::domain::treasury::UserWalletId;
use crate::interfaces::http::{
    auth::extractor::AuthUser, error::AppError, wallet::dto::TransferRequest,
};
use crate::setup::state::SharedState;

pub async fn transfer_to_user_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<TransferRequest>,
) -> Result<Json<bool>, AppError> {
    let amount = BigDecimal::from_str(&req.amount)
        .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;

    state
        .treasury_service
        .transfer_funds
        .execute(TransferFundsInput {
            user_id: user.user_id,
            sender_wallet_id: UserWalletId(req.sender_wallet_id),
            receiver_address: req.receiver_address,
            amount,
        })
        .map_err(AppError::from)?;

    Ok(Json(true))
}
