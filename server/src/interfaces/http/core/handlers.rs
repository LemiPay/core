use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::settlements::get_settlements::dto::GetSettlementsInput;
use crate::application::settlements::pay_settlement::dto::PaySettlementInput;
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::core::dto::{
    GetSettlementsResponse, PaySettlementRequest, PaySettlementResponse,
};
use crate::interfaces::http::transaction::dto::TransactionTypeResponse;
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

pub async fn pay_settlement(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(body): Json<PaySettlementRequest>,
) -> Result<Json<PaySettlementResponse>, AppError> {
    let input = PaySettlementInput {
        user_id: user.user_id,
        group_id: GroupId(group_id),
        amount: body.amount,
        address: body.address,
        currency_id: CurrencyId(body.currency_id),
        description: body.description,
    };
    let result = state.settlements_service.pay_settlement.execute(input)?;
    let tx = result.transaction;
    Ok(Json(PaySettlementResponse {
        id: tx.id,
        amount: tx.amount,
        user_id: tx.user_id,
        group_id: tx.group_id,
        currency_id: tx.currency_id,
        address: tx.address,
        description: tx.description,
        tx_type: TransactionTypeResponse::from(tx.tx_type),
        created_at: tx.created_at,
    }))
}
