use crate::data::state::SharedState;
use crate::handlers::auth::{get_me, login, register};
use crate::security::middlewares::auth::auth_middleware;
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn auth_routes(state: SharedState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/me",
            get(get_me).route_layer(middleware::from_fn(auth_middleware)),
        )
        .with_state(state)
}
