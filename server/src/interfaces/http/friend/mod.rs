use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

use crate::interfaces::http::{
    friend::handlers::{
        block_user, list_friends, list_received_requests, list_sent_requests, respond_request,
        search_users, send_request, unfriend,
    },
    middlewares::auth_middleware::auth_middleware,
};
use crate::setup::state::SharedState;

pub mod dto;
pub mod handlers;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/request/{user_id}", post(send_request))
        .route("/respond/{user_id}", post(respond_request))
        .route("/list", get(list_friends))
        .route("/requests/received", get(list_received_requests))
        .route("/requests/sent", get(list_sent_requests))
        .route("/search", get(search_users))
        .route("/{user_id}", delete(unfriend))
        .route("/block/{user_id}", post(block_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
