use axum::{Router, middleware, routing::get};

use crate::{
    interfaces::http::{
        core::handlers::get_balances,
        middlewares::{
            auth_middleware::auth_middleware, group_guard_middleware::is_in_group_middleware,
        },
    },
    setup::state::SharedState,
};

pub mod dto;
pub mod handlers;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/balances/{group_id}", get(get_balances))
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
