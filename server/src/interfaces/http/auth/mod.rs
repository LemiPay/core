use crate::setup::state::AppState;
use axum::{Router, routing::post};

mod dto;
pub(crate) mod extractor;
mod handlers;

use handlers::{login::login, register::register};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}
