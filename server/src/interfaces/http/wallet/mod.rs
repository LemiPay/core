use axum::{
    Router, middleware,
    routing::{get, post},
};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::auth_middleware::auth_middleware;

use handlers::{
    create_wallet::create_wallet, faucet_fund_wallet::faucet_fund_wallet,
    faucet_withdraw_wallet::faucet_withdraw_wallet,
    get_wallet::get_my_wallet_by_address_and_ticker, list_wallets::get_all_wallets,
    transfer::transfer_to_user_wallet,
};

use crate::setup::state::SharedState;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/create", post(create_wallet))
        .route("/fund/{wallet_id}", post(faucet_fund_wallet))
        .route("/withdraw/{wallet_id}", post(faucet_withdraw_wallet))
        .route("/transfer", post(transfer_to_user_wallet))
        .route("/get-all", get(get_all_wallets))
        .route("/{address}", get(get_my_wallet_by_address_and_ticker))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
