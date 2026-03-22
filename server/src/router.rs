use axum::Router;
use crate::data::state::{SharedState};
use crate::routes::user::user_routes;
use crate::routes::auth::auth_routes;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;




pub fn create_router(state: SharedState) -> Router {

    let cors = CorsLayer::new()
        // Permitimos que el front en el puerto 5173 nos hable
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        // Permitimos los métodos que necesites (POST para registro, etc)
        .allow_methods([Method::POST, Method::GET])
        // Muy importante: permitir el encabezado Content-Type para el JSON
        .allow_headers([axum::http::header::CONTENT_TYPE]);
    Router::new()
        .merge(user_routes(state.clone()))
        .nest("/auth", auth_routes(state.clone()))
        .layer(cors)//este layer tiene que ir al final de la creacion del Router por si dsp hay que agregar otros nest
}