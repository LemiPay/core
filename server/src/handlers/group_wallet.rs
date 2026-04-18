use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::group::group_wallet::GroupWallet;
use crate::models::proposals::fund_round::FundProposalExpanded;
use crate::models::transaction::fund_round_contrib::FundRoundContribution;
use crate::security::auth_extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateFundRoundRequest {
    pub target_amount: String,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct ContributeFundRoundRequest {
    pub amount: String,
    pub sender_wallet_id: Uuid,
}

#[derive(Serialize)]
pub struct FundRoundStatusResponse {
    pub fund_round: FundProposalExpanded,
    pub total_contributed: String,
    pub target_amount: String,
    pub is_completed: bool,
}

pub async fn create_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<CreateFundRoundRequest>,
) -> Result<Json<FundProposalExpanded>, AppError> {
    let result = state
        .group_wallet_service
        .create_fund_round(user.user_id, group_id, payload)?;
    Ok(Json(result))
}

pub async fn contribute_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
    Json(payload): Json<ContributeFundRoundRequest>,
) -> Result<Json<FundRoundStatusResponse>, AppError> {
    let result =
        state
            .group_wallet_service
            .contribute_fund_round(user.user_id, fund_round_id, payload)?;
    Ok(Json(result))
}

pub async fn get_user_contrib(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundContribution>, AppError> {
    let result = state
        .group_wallet_service
        .get_user_contribution(user.user_id, fund_round_id)?;
    Ok(Json(result))
}

pub async fn get_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundStatusResponse>, AppError> {
    let result = state
        .group_wallet_service
        .get_fund_round_status(user.user_id, fund_round_id)?;
    Ok(Json(result))
}

pub async fn cancel_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundProposalExpanded>, AppError> {
    let result = state
        .group_wallet_service
        .cancel_fund_round(user.user_id, fund_round_id)?;
    Ok(Json(result))
}

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
