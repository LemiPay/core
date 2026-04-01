use crate::data::state::SharedState;
use crate::handlers::proposal::{delete_proposal, group_proposals, my_proposals, new_group_member};
use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};
use axum::routing::{delete, get, post};
use axum::{Router, middleware};

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
        .route(
            "/{proposal_id}",
            delete(delete_proposal), // put(update_proposal)
                                     // El put hay que repensarlo, tiene varias opciones
        )
        .with_state(state)
}
