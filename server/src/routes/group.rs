use crate::data::state::SharedState;
use crate::handlers::group::create_group;
use crate::security::auth_middleware::auth_middleware;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn group_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/create",
            post(create_group).route_layer(middleware::from_fn(auth_middleware)),
        )
        .with_state(state)
}
