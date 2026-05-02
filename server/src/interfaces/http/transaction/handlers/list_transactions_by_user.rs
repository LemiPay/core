use axum::{Json, extract::State};

use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::error::AppError;
use crate::interfaces::http::transaction::dto::TransactionResponse;
use crate::setup::state::SharedState;

pub async fn list_transactions_by_user(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    let txs = state
        .treasury_service
        .list_user_transactions
        .execute(user.user_id)
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
