use crate::data::state::SharedState;
use crate::handlers::group_wallet::{
    cancel_fund_round, contribute_fund_round, create_fund_round, create_group_wallet,
    get_fund_round, get_group_wallets,
};
use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};
use axum::routing::{delete, get, post};
use axum::{Router, middleware};

pub fn group_wallet_routes(state: SharedState) -> Router {
    Router::new()
        // Create group wallet
        .route(
            "/{group_id}/create",
            post(create_group_wallet).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        // Find group wallets
        .route(
            "/{group_id}",
            get(get_group_wallets).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        // Fund Rounds
        // Create fund round
        .route(
            "/fund-round/create/{group_id}",
            post(create_fund_round).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        // Contribute
        .route(
            "/fund-round/{fund_round_id}/contribute",
            post(contribute_fund_round),
        )
        // Find fund round
        .route("/fund-round/{fund_round_id}", get(get_fund_round))
        // Cancel fund round
        .route(
            "/fund-round/{fund_round_id}/cancel",
            delete(cancel_fund_round),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
