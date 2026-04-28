use crate::setup::state::AppState;
use axum::routing::get;
use axum::{Router, middleware};

pub mod dto;
pub mod handlers;

use crate::interfaces::http::middlewares::auth_middleware::auth_middleware;
use handlers::{get_user::get_user, me::get_me};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/id/{id}", get(get_user))
        .route(
            "/me",
            get(get_me).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .with_state(state)
}
