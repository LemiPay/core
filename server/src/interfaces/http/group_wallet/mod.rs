use axum::{
    Router, middleware,
    routing::{get, post},
};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware,
    group_guard_middleware::{is_group_admin_middleware, is_in_group_middleware},
};

use handlers::{create_group_wallet::create_group_wallet, list_group_wallets::list_group_wallets};

use crate::setup::state::SharedState;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route(
            "/{group_id}/create",
            post(create_group_wallet).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/{group_id}",
            get(list_group_wallets).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
