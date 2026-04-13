use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::group_wallet::{create_group_wallet, get_group_wallet_balance};

use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};

pub fn group_wallet_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/{group_id}/create",
            post(create_group_wallet).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route(
            "/{group_id}/balance",
            get(get_group_wallet_balance).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
