use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::settlements::get_settlements::dto::GetSettlementsInput;
use crate::domain::group::GroupId;
use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::core::dto::GetSettlementsResponse;
use crate::{
    interfaces::http::{core::dto::BalancesResponse, error::AppError},
    setup::state::SharedState,
};

pub async fn get_balances(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<BalancesResponse>, AppError> {
    let result = state
        .balances_service
        .get_balances(GroupId(group_id))
        .map_err(AppError::from)?;
    Ok(Json(result.into()))
}

pub async fn get_settlements(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    _user: AuthUser,
) -> Result<Json<GetSettlementsResponse>, AppError> {
    let input = GetSettlementsInput {
        group_id: GroupId(group_id),
    };
    let result = state.settlements_service.get_settlements.execute(input)?;
    Ok(Json(result.into()))
}
