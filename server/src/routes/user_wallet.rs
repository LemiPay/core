use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};

use crate::data::state::SharedState;
use crate::handlers::user_wallet::{create_wallet, get_wallet};

use crate::security::middlewares::auth::auth_middleware;

pub fn user_wallet_routes(state: SharedState) -> Router {
    Router::new()
        .route("/{user_id}", get(get_wallet))
        .route("/create", post(create_wallet))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
