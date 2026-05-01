use crate::setup::state::SharedState;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};

use tower_http::cors::CorsLayer;

use crate::interfaces::http::{auth, governance, group, group_wallet, transaction, users, wallet};

pub fn create_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        // Permitimos que el front en el puerto 5173 nos hable
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        // HTTP Methods Permitidos
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        // Headers Permitidos
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/user", users::routes(state.clone()))
        .nest("/group", group::routes(state.clone()))
        .nest("/wallet", wallet::routes(state.clone()))
        .nest("/governance", governance::routes(state.clone()))
        .nest("/group-wallet", group_wallet::routes(state.clone()))
        .nest("/transaction", transaction::routes(state.clone()))
        .layer(cors)
        .with_state(state)
}
