use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::proposal::Proposal;
use crate::models::proposals::new_member::NewMemberProposalExpanded;
use crate::security::auth_extractor::AuthUser;
use axum::extract::Path;
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

#[derive(Deserialize)]
pub struct NewMemberRequest {
    pub user_id: Option<Uuid>,
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
            .create_new_member_proposal(user.user_id, group_id, payload.user_id);
    Ok(Json(new_proposal?))
}

pub async fn delete_proposal(
    State(state): State<SharedState>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<Proposal>, AppError> {
    let delete_proposal = state.proposal_service.logic_proposal_delete(proposal_id)?;
    Ok(Json(delete_proposal))
}
