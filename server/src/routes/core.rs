use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::core::get_balances;
use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::is_in_group_middleware;

pub fn core_routes(state: SharedState) -> Router {
    Router::new()
        .route("/balances/{group_id}", get(get_balances))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            is_in_group_middleware,
        ))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
