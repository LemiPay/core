use crate::data::state::SharedState;
use crate::handlers::user::{get_user, list_users};
use crate::security::middlewares::auth::auth_middleware;
use axum::{Router, middleware, routing::get};

pub fn user_routes(state: SharedState) -> Router {
    Router::new()
        .route(
            "/users",
            get(list_users).route_layer(middleware::from_fn(auth_middleware)),
        )
        //.route_layer(middleware::from_fn(auth_middleware))
        .route("/users/{id}", get(get_user))
        .with_state(state)
}
