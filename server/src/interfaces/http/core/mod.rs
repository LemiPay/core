use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::interfaces::http::core::handlers::{get_balances, get_settlements, pay_settlement};
use crate::{
    interfaces::http::middlewares::{
        auth_middleware::auth_middleware, group_guard_middleware::is_in_group_middleware,
    },
    setup::state::SharedState,
};

pub mod dto;
pub mod handlers;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/balances/{group_id}", get(get_balances))
        .route("/get-settlements/{group_id}", get(get_settlements))
        .route("/pay-settlement/{group_id}", post(pay_settlement))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            is_in_group_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
