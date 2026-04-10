use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user_wallet::{NewUserWallet, UserWallet};

use crate::security::auth_extractor::AuthUser;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct NewWalletRequest {
    pub address: String,
    pub currency_ticker: String,
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

pub async fn get_wallet(
    State(state): State<SharedState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<String>, AppError> {
    let result = state
        .user_wallet_service
        .get_another_user_wallet_address(user_id)?;
    Ok(Json(result))
}
