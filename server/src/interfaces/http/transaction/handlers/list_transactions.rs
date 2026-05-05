use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::domain::group::GroupId;
use crate::interfaces::http::{error::AppError, transaction::dto::TransactionResponse};
use crate::setup::state::SharedState;

pub async fn list_transactions(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    let txs = state
        .treasury_service
        .list_group_transactions
        .execute(GroupId(group_id))
        .map_err(AppError::from)?;

    Ok(Json(
        txs.into_iter()
            .map(|t| TransactionResponse {
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
            })
            .collect(),
    ))
}
