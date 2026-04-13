use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::proposals::withdraw::WithdrawProposalExpanded;
use crate::models::transaction::Transaction;
use crate::security::auth_extractor::AuthUser;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FundGroupRequest {
    pub amount: BigDecimal,
    pub currency_id: Uuid,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TransactionIdQuery {
    pub transaction_id: Uuid,
}

#[derive(Deserialize)]
pub struct WithdrawProposalRequest {
    pub amount: BigDecimal,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct ExecuteWithdrawRequest {
    pub proposal_id: Uuid,
}

pub async fn fund_group(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<FundGroupRequest>,
) -> Result<Json<Transaction>, AppError> {
    let result = state
        .transaction_service
        .fund_group(user.user_id, group_id, payload)?;
    Ok(Json(result))
}

pub async fn create_withdraw_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<WithdrawProposalRequest>,
) -> Result<Json<WithdrawProposalExpanded>, AppError> {
    let result =
        state
            .transaction_service
            .create_withdraw_proposal(user.user_id, group_id, payload)?;
    Ok(Json(result))
}

pub async fn execute_withdraw_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<ExecuteWithdrawRequest>,
) -> Result<Json<Transaction>, AppError> {
    let result = state
        .transaction_service
        .execute_withdraw(user.user_id, group_id, payload)?;
    Ok(Json(result))
}

pub async fn list_transactions(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    let result = state.transaction_service.list_by_group(group_id)?;
    Ok(Json(result))
}

pub async fn get_transaction(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Query(params): Query<TransactionIdQuery>,
) -> Result<Json<Transaction>, AppError> {
    let result = state
        .transaction_service
        .get_by_id(params.transaction_id, group_id)?;
    Ok(Json(result))
}
