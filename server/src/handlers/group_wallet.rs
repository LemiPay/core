use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::group::group_wallet::GroupWallet;
use axum::Json;
use axum::extract::{Path, State};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewGroupWalletRequest {
    pub address: String,
    pub currency_ticker: String,
}

pub async fn create_group_wallet(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<NewGroupWalletRequest>,
) -> Result<Json<GroupWallet>, AppError> {
    let wallet = state
        .group_wallet_service
        .create_wallet(payload, group_id)?;
    Ok(Json(wallet))
}

pub async fn get_group_wallets(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<GroupWallet>>, AppError> {
    let wallets = state.group_wallet_service.get_wallets_by_group(group_id)?;
    Ok(Json(wallets))
}
