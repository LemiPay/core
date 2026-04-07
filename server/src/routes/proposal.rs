use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::proposal::{
    delete_proposal, group_proposals, my_proposals, new_group_member, received_proposals,
    respond_to_user_proposal,
};

use crate::security::middlewares::auth::auth_middleware;
use crate::security::middlewares::is_in_group::{
    is_group_admin_middleware, is_in_group_middleware,
};

pub fn proposal_routes(state: SharedState) -> Router {
    Router::new()
        // Get proposals created by Me
        .route("/my", get(my_proposals))
        // Get proposals sent to Me
        .route("/received", get(received_proposals))
        // Get Proposals by Group
        .route(
            "/group/{id}",
            get(group_proposals).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_in_group_middleware,
            )),
        )
        // Create NewMember Proposal
        .route(
            "/new-member/{group_id}", // Para el middleware
            post(new_group_member).route_layer(middleware::from_fn_with_state(
                state.clone(),
                is_group_admin_middleware,
            )),
        )
        // Delete Proposal
        .route(
            "/{group_id}", // Para el middleware
            delete(delete_proposal) // put(update_proposal)
                // El put hay que repensarlo, tiene varias opciones
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    is_group_admin_middleware,
                )),
        )
        .route(
            "/respond_proposal/{proposal_id}",
            put(respond_to_user_proposal),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
