use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::domain::group::GroupId;
use crate::interfaces::http::{error::AppError, group_wallet::dto::GroupWalletResponse};
use crate::setup::state::SharedState;

pub async fn list_group_wallets(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<GroupWalletResponse>>, AppError> {
    let wallets = state
        .treasury_service
        .list_group_wallets
        .execute(GroupId(group_id))
        .map_err(AppError::from)?;

    Ok(Json(
        wallets
            .into_iter()
            .map(|w| GroupWalletResponse {
                id: w.id,
                address: w.address,
                group_id: w.group_id,
                currency_id: w.currency_id,
                balance: w.balance,
                created_at: w.created_at,
                updated_at: w.updated_at,
            })
            .collect(),
    ))
}
