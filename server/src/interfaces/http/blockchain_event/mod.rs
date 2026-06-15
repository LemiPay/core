use axum::{Router, middleware, routing::get};

pub mod handlers;

use crate::interfaces::http::middlewares::auth_middleware::auth_middleware;
use crate::setup::state::SharedState;

use handlers::list_events::list_events;

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/me", get(list_events))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
