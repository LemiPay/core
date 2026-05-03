use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::treasury::fund_group::dto::FundGroupInput;
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    transaction::dto::{FundGroupRequest, TransactionResponse},
};
use crate::setup::state::SharedState;

pub async fn fund_group(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(req): Json<FundGroupRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    let output = state
        .treasury_service
        .fund_group
        .execute(FundGroupInput {
            user_id: user.user_id,
            group_id: GroupId(group_id),
            amount: req.amount,
            address: req.address,
            currency_id: CurrencyId(req.currency_id),
            description: req.description,
        })
        .map_err(AppError::from)?;

    let t = output.transaction;
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
