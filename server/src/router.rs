use crate::data::state::SharedState;

use axum::Router;
use axum::http::{
    HeaderValue, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::CorsLayer;

// Routes
use crate::routes::auth::auth_routes;
use crate::routes::group::group_routes;
use crate::routes::proposal::proposal_routes;
use crate::routes::user::user_routes;

pub fn create_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        // Permitimos que el front en el puerto 5173 nos hable
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        // HTTP Methods Permitidos
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        // Headers Permitidos
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    Router::new()
        .merge(user_routes(state.clone()))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/group", group_routes(state.clone()))
        .nest("/proposal", proposal_routes(state.clone()))
        .nest("/wallet", user_routes(state.clone()))
        .layer(cors) //este layer tiene que ir al final de la creación del Router por si dsp hay que agregar otros nest
}
