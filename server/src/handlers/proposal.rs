use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::proposals::new_member::NewMemberProposalExpanded;
use crate::security::auth_extractor::AuthUser;
use axum::extract::Path;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GroupProposalRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

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
