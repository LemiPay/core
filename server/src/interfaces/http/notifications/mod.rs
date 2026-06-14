use axum::{Router, middleware, routing::get};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::auth_middleware::auth_middleware;
use crate::setup::state::SharedState;

use handlers::{get_channels, get_events, get_my_preferences, upsert_my_preference};

pub fn routes(state: SharedState) -> Router<SharedState> {
    let protected = Router::new()
        .route(
            "/preferences",
            get(get_my_preferences).post(upsert_my_preference),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/events", get(get_events))
        .route("/channels", get(get_channels))
        .merge(protected)
        .with_state(state)
}
