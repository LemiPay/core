use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::interfaces::http::middlewares::{
    auth_middleware::auth_middleware,
    group_guard_middleware::{is_group_admin_middleware, is_in_group_middleware},
};
use crate::setup::state::SharedState;

pub mod dto;
pub mod handlers;

use handlers::{add_permission, list_permissions, remove_permission};

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route(
            "/{group_id}",
            get(list_permissions).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{group_id}",
            post(add_permission).delete(remove_permission).route_layer(
                middleware::from_fn_with_state(state.clone(), is_group_admin_middleware),
            ),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
