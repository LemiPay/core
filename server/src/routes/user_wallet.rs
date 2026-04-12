use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::user_wallet::{
    create_wallet, faucet_fund_wallet, faucet_withdraw_wallet, get_all_wallets,
    get_my_wallet_by_address_and_ticker, transfer_to_user_wallet,
};

use crate::security::middlewares::auth::auth_middleware;

pub fn user_wallet_routes(state: SharedState) -> Router {
    Router::new()
        .route("/create", post(create_wallet))
        .route("/fund/{wallet_id}", post(faucet_fund_wallet))
        .route("/withdraw/{wallet_id}", post(faucet_withdraw_wallet))
        .route("/transfer", post(transfer_to_user_wallet))
        .route("/get-all", get(get_all_wallets))
        .route("/{address}", get(get_my_wallet_by_address_and_ticker))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
