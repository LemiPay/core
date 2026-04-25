use crate::setup::state::AppState;
use axum::{
    Router, middleware,
    routing::{get, post},
};

mod auth_middleware;
mod dto;
pub(crate) mod extractor;
mod handlers;

use handlers::{login::login, me::get_me, register::register};

use auth_middleware::auth_middleware;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/me",
            get(get_me).route_layer(middleware::from_fn(auth_middleware)),
        )
        .with_state(state)
}
