pub mod chat;
pub mod dto;

use axum::{Router, middleware, routing::post};

use crate::interfaces::http::middlewares::auth_middleware::auth_middleware;
use crate::setup::state::SharedState;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/ask", post(chat::ask))
        .route("/explain", post(chat::explain))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
