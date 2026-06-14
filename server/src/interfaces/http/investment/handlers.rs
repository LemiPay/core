use crate::domain::group::GroupId;
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    investment::dto::{
        CreateInvestmentProposalRequest, ExecuteInvestmentRequest, InvestmentProposalResponse,
        InvestmentResponse, InvestmentStrategyResponse, SnapshotResponse,
        WithdrawInvestmentRequest,
    },
};
use crate::setup::state::SharedState;

pub async fn list_approved_proposals(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<InvestmentProposalResponse>>, AppError> {
    let items = state
        .investment_service
        .list_approved_proposals(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn list_strategies(
    State(state): State<SharedState>,
) -> Result<Json<Vec<InvestmentStrategyResponse>>, AppError> {
    let items = state
        .investment_service
        .list_strategies()
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn create_investment_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<CreateInvestmentProposalRequest>,
) -> Result<Json<InvestmentProposalResponse>, AppError> {
    let item = state
        .investment_service
        .create_investment_proposal(
            user.user_id.0,
            group_id,
            payload.amount,
            payload.strategy_id,
            payload.currency_id,
        )
        .map_err(AppError::from)?;

    let group_name = "el grupo";
    state
        .notification_service
        .notify_group_event("investment_created", GroupId(group_id), group_name)
        .await;

    Ok(Json(item.into()))
}

pub async fn execute_investment_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<ExecuteInvestmentRequest>,
) -> Result<Json<InvestmentResponse>, AppError> {
    let item = state
        .investment_service
        .execute_investment_proposal(user.user_id.0, group_id, payload.proposal_id)
        .map_err(AppError::from)?;

    let group_name = "el grupo";
    state
        .notification_service
        .notify_group_event("proposal_executed", GroupId(group_id), group_name)
        .await;

    Ok(Json(item.into()))
}

pub async fn withdraw_investment(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<WithdrawInvestmentRequest>,
) -> Result<Json<InvestmentResponse>, AppError> {
    let item = state
        .investment_service
        .withdraw_investment(user.user_id.0, group_id, payload.investment_id)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn list_group_investments(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<InvestmentResponse>>, AppError> {
    let items = state
        .investment_service
        .list_group_investments(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn get_investment_snapshots(
    State(state): State<SharedState>,
    Path(investment_id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<Vec<SnapshotResponse>>, AppError> {
    let items = state
        .investment_service
        .list_snapshots(investment_id, user.user_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}
