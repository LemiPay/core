use axum::{Router, routing::get};

use crate::interfaces::http::health::handlers::health_check;
use crate::setup::state::SharedState;

pub mod dto;
pub mod handlers;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(health_check))
}
