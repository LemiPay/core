use axum::{
    Router, middleware,
    routing::{get, post},
};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware, group_guard_middleware::is_in_group_middleware,
};

use handlers::{
    fund_group::fund_group, get_transaction::get_transaction, list_transactions::list_transactions,
};

use crate::setup::state::SharedState;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/{group_id}/fund", post(fund_group))
        .route("/{group_id}/list", get(list_transactions))
        .route("/{group_id}/one", get(get_transaction))
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
