use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::security::auth_extractor::AuthUser;

use axum::{
    Json,
    extract::{Path, State},
};

use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserBalance {
    pub user_name: String,
    pub user_id: Uuid,
    pub balance: BigDecimal,
}

#[derive(Serialize)]
pub struct Balances {
    pub group_balance: BigDecimal,
    pub balances: Vec<UserBalance>,
}

pub async fn get_balances(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    _user: AuthUser,
) -> Result<Json<Balances>, AppError> {
    let result = state.core_service.get_balances(group_id)?;
    Ok(Json(result))
}
