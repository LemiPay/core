use crate::data::state::SharedState;
use crate::handlers::group::{create_group, get_group_by_id, make_group_admin};
use crate::security::auth_middleware::auth_middleware;
use crate::security::is_in_group_middleware::{is_group_admin_middleware, is_in_group_middleware};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn group_routes(state: SharedState) -> Router {
    Router::new()
        .route("/create", post(create_group))
        .route(
            "/{id}",
            get(get_group_by_id).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        .route(
            "/{id}/make_admin",
            post(make_group_admin).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
