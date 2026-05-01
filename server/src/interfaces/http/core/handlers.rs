use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

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
        .get_balances(group_id)
        .map_err(AppError::from)?;
    Ok(Json(result.into()))
}
