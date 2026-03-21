use axum::{
    Router,
    routing::{post},
};
use crate::data::state::SharedState;
use crate::handlers::auth::{register};

pub fn auth_routes(state: SharedState) -> Router {
    Router::new()
        .route("/register", post(register))
        //.route("/login", get(get_user))
        .with_state(state)
}