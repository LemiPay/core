use crate::data::state::SharedState;
use crate::handlers::proposal::{group_proposals, my_proposals, new_group_member};
use crate::security::auth_middleware::auth_middleware;
use crate::security::is_in_group_middleware::{is_group_admin_middleware, is_in_group_middleware};
use axum::routing::post;
use axum::{Router, middleware, routing::get};

pub fn proposal_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/my",
            get(my_proposals).route_layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/group/{id}",
            get(group_proposals)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_in_group_middleware,
                ))
                .route_layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/new-member/{group_id}",
            post(new_group_member)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_group_admin_middleware,
                ))
                .route_layer(middleware::from_fn(auth_middleware)),
        )
        .with_state(state)
}
