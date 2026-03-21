use axum::Router;
use crate::data::state::{SharedState};
use crate::routes::user::user_routes;
use crate::routes::auth::auth_routes;


pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .merge(user_routes(state.clone()))
        .nest("/auth", auth_routes(state.clone()))
}