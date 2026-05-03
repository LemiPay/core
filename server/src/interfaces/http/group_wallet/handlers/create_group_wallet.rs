use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::treasury::create_group_wallet::dto::CreateGroupWalletInput;
use crate::domain::group::GroupId;
use crate::interfaces::http::{
    error::AppError,
    group_wallet::dto::{CreateGroupWalletRequest, GroupWalletResponse},
};
use crate::setup::state::SharedState;

pub async fn create_group_wallet(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Json(req): Json<CreateGroupWalletRequest>,
) -> Result<Json<GroupWalletResponse>, AppError> {
    let output = state
        .treasury_service
        .create_group_wallet
        .execute(CreateGroupWalletInput {
            group_id: GroupId(group_id),
            address: req.address,
            currency_ticker: req.currency_ticker,
        })
        .map_err(AppError::from)?;

    let w = output.wallet;
    Ok(Json(GroupWalletResponse {
        id: w.id,
        address: w.address,
        group_id: w.group_id,
        currency_id: w.currency_id,
        balance: w.balance,
        created_at: w.created_at,
        updated_at: w.updated_at,
    }))
}
