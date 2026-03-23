use crate::data::state::SharedState;
use crate::handlers::auth::{login, register};
use axum::{Router, routing::post};

pub fn auth_routes(state: SharedState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}
