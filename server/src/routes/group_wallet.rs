use axum::routing::{get, post, put};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::group_wallet::{
    cancel_fund_round, contribute_fund_round, create_fund_round, get_fund_round,
};
use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::is_in_group_middleware;

pub fn group_wallet_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/fund-round/create/{group_id}",
            post(create_fund_round).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/fund-round/{fund_round_id}/contribute",
            post(contribute_fund_round),
        )
        .route("/fund-round/{fund_round_id}", get(get_fund_round))
        .route(
            "/fund-round/{fund_round_id}/cancel",
            put(cancel_fund_round),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
