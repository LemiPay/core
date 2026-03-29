use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user::User;
use crate::security::auth_extractor::AuthUser;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GroupProposalRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct GroupProposalResponse {
    token: String,
}

pub async fn group_proposals(
    State(state): State<SharedState>,
    Json(payload): Json<GroupProposalRequest>,
) -> Result<Json<GroupProposalResponse>, AppError> {
    Ok(Json(GroupProposalResponse {
        token: "".parse().unwrap(),
    }))
}

#[derive(Serialize)]
pub struct MyProposalsResponse {
    id: Uuid,
}

pub async fn my_proposals(user: AuthUser) -> Result<Json<MyProposalsResponse>, AppError> {
    Ok(Json(MyProposalsResponse { id: user.user_id }))
}
