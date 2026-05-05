use axum::{Json, extract::State};
use std::collections::HashMap;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::dto::{AddressGroupResponse, WalletWithTickerResponse},
};
use crate::setup::state::SharedState;

pub async fn get_all_wallets(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<AddressGroupResponse>>, AppError> {
    let wallets = state
        .treasury_service
        .list_user_wallets
        .execute(user.user_id)
        .map_err(AppError::from)?;

    let mut grouped: HashMap<String, Vec<WalletWithTickerResponse>> = HashMap::new();

    for w in wallets {
        let entry = grouped.entry(w.address.clone()).or_default();
        entry.push(WalletWithTickerResponse {
            wallet_id: w.wallet_id,
            address: w.address,
            balance: w.balance,
            currency_id: w.currency_id,
            ticker: w.ticker,
        });
    }

    let response: Vec<AddressGroupResponse> = grouped
        .into_iter()
        .map(|(address, currencies)| AddressGroupResponse {
            address,
            currencies,
        })
        .collect();

    Ok(Json(response))
}
