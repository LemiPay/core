use axum::{Json, extract::State};

use crate::interfaces::http::auth::extractor::AuthUser;
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;

pub async fn list_events(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<crate::application::treasury::dto::BlockchainEventDetails>>, AppError> {
    let wallets = state
        .treasury_service
        .list_user_wallets
        .execute(user.user_id)
        .map_err(|_| AppError::Internal)?;

    let addresses: Vec<String> = wallets.iter().map(|w| w.address.clone()).collect();

    if addresses.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let events = state
        .fund_event_repo
        .list_by_wallet_addresses(&addresses)
        .map_err(|_| AppError::Internal)?;

    Ok(Json(events))
}
