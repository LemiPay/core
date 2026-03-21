use axum::{
    Router,
    routing::{get},
};
use crate::data::state::SharedState;
use crate::handlers::user::{get_user, list_users};

pub fn user_routes(state: SharedState) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user))
        .with_state(state)
}
