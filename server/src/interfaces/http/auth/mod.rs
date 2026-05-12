use crate::setup::state::SharedState;
use axum::{Router, routing::post};

mod dto;
pub(crate) mod extractor;
mod handlers;

use crate::interfaces::http::auth::handlers::verify_challenge::verify_challenge;
use handlers::{generate_challenge::generate_challenge, login::login, register::register};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/request-challenge", post(generate_challenge))
        .route("/verify-challenge", post(verify_challenge))
}
