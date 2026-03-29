use crate::data::state::SharedState;
use crate::handlers::proposal::{group_proposals, my_proposals};
use crate::security::auth_middleware::auth_middleware;
use axum::{Router, middleware, routing::get};

pub fn proposal_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/my",
            get(my_proposals).route_layer(middleware::from_fn(auth_middleware)),
        )
        .route("/group/{id}", get(group_proposals))
        .with_state(state)
}
