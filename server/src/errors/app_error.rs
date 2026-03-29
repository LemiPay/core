use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

use crate::data::error::DbError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Db(#[from] DbError),

    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,

    #[error("Password hashing error")]
    PasswordHash(String),

    #[error("Invalid credentials")]
    Unauthorized,
    #[error("user is already an admin")]
    AlreadyAdmin,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::PasswordHash(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::AlreadyAdmin => (StatusCode::FORBIDDEN, self.to_string()),
        };

        let body = Json(ErrorResponse { message });

        (status, body).into_response()
    }
}
