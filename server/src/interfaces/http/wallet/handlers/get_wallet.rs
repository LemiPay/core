use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::dto::{CurrencyQuery, UserWalletResponse},
};
use crate::setup::state::SharedState;

pub async fn get_my_wallet_by_address_and_ticker(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(address): Path<String>,
    Query(query): Query<CurrencyQuery>,
) -> Result<Json<UserWalletResponse>, AppError> {
    let details = state
        .treasury_service
        .get_user_wallet_by_address_and_ticker
        .execute(user.user_id, &address, &query.currency)
        .map_err(AppError::from)?;

    Ok(Json(UserWalletResponse {
        id: details.id,
        address: details.address,
        user_id: details.user_id,
        currency_id: details.currency_id,
        balance: details.balance,
        created_at: details.created_at,
        updated_at: details.updated_at,
    }))
}
