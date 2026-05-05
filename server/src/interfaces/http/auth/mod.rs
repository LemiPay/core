use crate::setup::state::SharedState;
use axum::{Router, routing::post};

mod dto;
pub(crate) mod extractor;
mod handlers;

use handlers::{login::login, register::register};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
