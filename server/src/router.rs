use axum::Router;

use crate::handlers::user::AppState;
use crate::routes::user::user_routes;

pub fn create_router(state: AppState) -> Router {
    Router::new().merge(user_routes(state))
}
