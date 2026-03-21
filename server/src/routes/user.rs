use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::user::{AppState, create_user, get_user, list_users};

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user))
        .with_state(state)
}
