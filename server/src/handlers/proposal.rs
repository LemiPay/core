use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::proposal::Proposal;
use crate::models::proposals::new_member::{
    NewMemberProposalExpanded, ReceivedNewMemberProposalExpanded,
};
use crate::models::proposals::withdraw::WithdrawProposalExpanded;
use crate::security::auth_extractor::AuthUser;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ProposalsResponse {
    proposals: Vec<NewMemberProposalExpanded>,
}

pub async fn group_proposals(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<ProposalsResponse>, AppError> {
    let proposals = state.proposal_service.get_proposals_group(group_id)?;
    Ok(Json(ProposalsResponse { proposals }))
}

pub async fn my_proposals(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<ProposalsResponse>, AppError> {
    let proposals = state.proposal_service.get_my_proposals(user.user_id)?;
    Ok(Json(ProposalsResponse { proposals }))
}

pub async fn received_proposals(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<ReceivedNewMemberProposalExpanded>>, AppError> {
    let proposals = state
        .proposal_service
        .get_received_proposals(user.user_id)?;
    Ok(Json(proposals))
}

#[derive(Deserialize)]
pub struct NewMemberRequest {
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
}

#[derive(Deserialize)]
pub struct RespondToNewMemberRequest {
    pub response: bool,
}

pub async fn new_group_member(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<NewMemberRequest>,
) -> Result<Json<NewMemberProposalExpanded>, AppError> {
    let new_proposal =
        state
            .proposal_service
            .create_new_member_proposal(user.user_id, group_id, payload);

    Ok(Json(new_proposal?))
}

#[derive(Deserialize)]
pub struct ProposalParams {
    proposal_id: Uuid,
}

pub async fn delete_proposal(
    State(state): State<SharedState>,
    Query(params): Query<ProposalParams>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Proposal>, AppError> {
    let delete_proposal = state
        .proposal_service
        .logic_proposal_delete(params.proposal_id, group_id)?;
    Ok(Json(delete_proposal))
}

pub async fn respond_to_user_proposal(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<RespondToNewMemberRequest>,
) -> Result<Json<NewMemberProposalExpanded>, AppError> {
    let update_proposal =
        state
            .proposal_service
            .respond_new_member_proposal(user.user_id, proposal_id, payload)?;
    Ok(Json(update_proposal))
}

pub async fn get_all_withdraw_proposals(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<WithdrawProposalExpanded>>, AppError> {
    let withdraw_proposals = state
        .proposal_service
        .get_all_withdraw_proposals(group_id)?
        .unwrap_or_default();
    Ok(Json(withdraw_proposals))
}
