use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::domain::group::GroupId;
use crate::domain::treasury::TransactionId;
use crate::interfaces::http::{
    error::AppError,
    transaction::dto::{TransactionIdQuery, TransactionResponse},
};
use crate::setup::state::SharedState;

pub async fn get_transaction(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Query(params): Query<TransactionIdQuery>,
) -> Result<Json<TransactionResponse>, AppError> {
    let t = state
        .treasury_service
        .get_group_transaction
        .execute(GroupId(group_id), TransactionId(params.transaction_id))
        .map_err(AppError::from)?;

    Ok(Json(TransactionResponse {
        id: t.id,
        tx_hash: t.tx_hash,
        amount: t.amount,
        user_id: t.user_id,
        group_id: t.group_id,
        currency_id: t.currency_id,
        address: t.address,
        description: t.description,
        tx_type: t.tx_type.into(),
        created_at: t.created_at,
        updated_at: t.updated_at,
    }))
}
