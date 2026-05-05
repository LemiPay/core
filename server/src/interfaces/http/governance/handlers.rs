use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    governance::dto::{
        ContributeFundRoundRequest, CreateFundRoundRequest, ExecuteWithdrawRequest,
        FundRoundContributionResponse, FundRoundProposalResponse, FundRoundRemainingResponse,
        FundRoundStatusResponse, NewMemberProposalResponse, NewMemberRequest, ProposalResponse,
        ReceivedNewMemberProposalResponse, RespondProposalRequest, WithdrawProposalRequest,
        WithdrawProposalResponse,
    },
};
use crate::setup::state::SharedState;

#[derive(serde::Deserialize)]
pub struct ProposalParams {
    proposal_id: Uuid,
}

pub async fn group_new_member_proposals(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<NewMemberProposalResponse>>, AppError> {
    let items = state
        .governance_service
        .list_group_new_member_proposals(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn my_new_member_proposals(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<NewMemberProposalResponse>>, AppError> {
    let items = state
        .governance_service
        .list_my_new_member_proposals(user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn received_new_member_proposals(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<ReceivedNewMemberProposalResponse>>, AppError> {
    let items = state
        .governance_service
        .list_received_new_member_proposals(user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn create_new_member_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<NewMemberRequest>,
) -> Result<Json<NewMemberProposalResponse>, AppError> {
    let item = state
        .governance_service
        .create_new_member_proposal(
            user.user_id.0,
            group_id,
            payload.user_id,
            payload.user_email,
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn respond_new_member_proposal(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<RespondProposalRequest>,
) -> Result<Json<NewMemberProposalResponse>, AppError> {
    let item = state
        .governance_service
        .respond_new_member_proposal(user.user_id.0, proposal_id, payload.response)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn delete_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Query(params): Query<ProposalParams>,
) -> Result<Json<ProposalResponse>, AppError> {
    let item = state
        .governance_service
        .cancel_proposal(params.proposal_id, group_id)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn create_withdraw_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<WithdrawProposalRequest>,
) -> Result<Json<WithdrawProposalResponse>, AppError> {
    let item = state
        .governance_service
        .create_withdraw_proposal(
            user.user_id.0,
            group_id,
            payload.address,
            payload.amount,
            payload.currency_id,
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn execute_withdraw_proposal(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<ExecuteWithdrawRequest>,
) -> Result<Json<ProposalResponse>, AppError> {
    state
        .governance_service
        .execute_withdraw_proposal(
            user.user_id.0,
            group_id,
            payload.address,
            payload.proposal_id,
            payload.currency_id,
        )
        .map_err(AppError::from)?;
    let proposal = state
        .governance_service
        .find_proposal(payload.proposal_id)
        .map_err(AppError::from)?;
    Ok(Json(proposal.into()))
}

pub async fn get_all_withdraw_proposals(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<WithdrawProposalResponse>>, AppError> {
    let items = state
        .governance_service
        .list_withdraw_proposals(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn create_fund_round_proposal(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<CreateFundRoundRequest>,
) -> Result<Json<FundRoundProposalResponse>, AppError> {
    let item = state
        .governance_service
        .create_fund_round_proposal(
            user.user_id.0,
            group_id,
            payload.target_amount,
            payload.currency_id,
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn contribute_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
    Json(payload): Json<ContributeFundRoundRequest>,
) -> Result<Json<FundRoundContributionResponse>, AppError> {
    let item = state
        .governance_service
        .contribute_fund_round(
            user.user_id.0,
            fund_round_id,
            payload.amount,
            payload.sender_wallet_id,
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn get_fund_round_status(
    State(state): State<SharedState>,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundStatusResponse>, AppError> {
    let (round, total, completed) = state
        .governance_service
        .find_fund_round_status(fund_round_id)
        .map_err(AppError::from)?;
    Ok(Json(FundRoundStatusResponse {
        target_amount: round.target_amount.to_string(),
        total_contributed: total.to_string(),
        is_completed: completed,
        fund_round: round.into(),
    }))
}

pub async fn get_fund_round_remaining(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundRemainingResponse>, AppError> {
    let (remaining, is_last_contributor) = state
        .governance_service
        .find_fund_round_remaining(fund_round_id, user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(FundRoundRemainingResponse {
        remaining: remaining.to_string(),
        is_last_contributor,
    }))
}

pub async fn get_user_contribution(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundContributionResponse>, AppError> {
    let item = state
        .governance_service
        .get_user_fund_round_contribution(fund_round_id, user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn get_all_fund_rounds(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<FundRoundProposalResponse>>, AppError> {
    let items = state
        .governance_service
        .list_fund_rounds(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn cancel_fund_round(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(fund_round_id): Path<Uuid>,
) -> Result<Json<FundRoundProposalResponse>, AppError> {
    let item = state
        .governance_service
        .cancel_fund_round(user.user_id.0, fund_round_id)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}
