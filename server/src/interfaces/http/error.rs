use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

use crate::application::auth::error::AuthError;
use crate::application::group::create_group::CreateGroupError;
use crate::application::group::delete_group::DeleteGroupError;
use crate::application::group::get_group::GetGroupError;
use crate::application::group::get_group_members::GetGroupMembersError;
use crate::application::group::leave_group::LeaveGroupError;
use crate::application::group::list_user_groups::ListUserGroupsError;
use crate::application::group::make_group_admin::MakeGroupAdminError;
use crate::application::group::update_group::UpdateGroupError;
use crate::infrastructure::db::error::DbError;

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

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Core operation failed")]
    Core,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::PasswordHash(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Core => (StatusCode::CONFLICT, self.to_string()),
        };

        let body = Json(ErrorResponse { message });

        (status, body).into_response()
    }
}

// Mappings

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidEmail => AppError::BadRequest("Invalid email".into()),
            AuthError::InvalidName => AppError::BadRequest("Invalid name".into()),
            AuthError::EmailAlreadyExists => AppError::BadRequest("Email already exists".into()),
            AuthError::InvalidCredentials => AppError::Unauthorized,
            AuthError::InternalError => AppError::Internal,
        }
    }
}

impl From<CreateGroupError> for AppError {
    fn from(err: CreateGroupError) -> Self {
        match err {
            CreateGroupError::InvalidName => AppError::BadRequest("Invalid group name".into()),
            CreateGroupError::InvalidDescription => {
                AppError::BadRequest("Invalid group description".into())
            }
            CreateGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<GetGroupError> for AppError {
    fn from(err: GetGroupError) -> Self {
        match err {
            GetGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<LeaveGroupError> for AppError {
    fn from(err: LeaveGroupError) -> Self {
        match err {
            LeaveGroupError::NotFound => AppError::NotFound,
            LeaveGroupError::NotMember => {
                AppError::Forbidden("User is not a member of this group".into())
            }
            LeaveGroupError::LastAdminCannotLeave => {
                AppError::BadRequest("El grupo tiene que tener al menos un administrador".into())
            }
            LeaveGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<ListUserGroupsError> for AppError {
    fn from(err: ListUserGroupsError) -> Self {
        match err {
            ListUserGroupsError::InternalError => AppError::Internal,
        }
    }
}

impl From<MakeGroupAdminError> for AppError {
    fn from(err: MakeGroupAdminError) -> Self {
        match err {
            MakeGroupAdminError::Forbidden => AppError::Forbidden("Forbidden".into()),
            MakeGroupAdminError::NotFound => AppError::NotFound,
            MakeGroupAdminError::BadRequest(message) => AppError::BadRequest(message),
            MakeGroupAdminError::Internal => AppError::Internal,
        }
    }
}

impl From<UpdateGroupError> for AppError {
    fn from(err: UpdateGroupError) -> Self {
        match err {
            UpdateGroupError::Forbidden => {
                AppError::Forbidden("Solo el administrador puede actualizar el grupo.".into())
            }
            UpdateGroupError::NotFound => AppError::NotFound,
            UpdateGroupError::BadRequest(message) => AppError::BadRequest(message),
            UpdateGroupError::Internal => AppError::Internal,
        }
    }
}

impl From<DeleteGroupError> for AppError {
    fn from(err: DeleteGroupError) -> Self {
        match err {
            DeleteGroupError::Forbidden => {
                AppError::Forbidden("Solo el administrador puede borrar el grupo".into())
            }
            DeleteGroupError::NotFound => AppError::NotFound,
            DeleteGroupError::Internal => AppError::Internal,
        }
    }
}

impl From<GetGroupMembersError> for AppError {
    fn from(err: GetGroupMembersError) -> Self {
        match err {
            GetGroupMembersError::Forbidden => AppError::Forbidden("Forbidden".into()),
            GetGroupMembersError::Internal => AppError::Internal,
        }
    }
}
