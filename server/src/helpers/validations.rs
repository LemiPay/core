use crate::errors::app_error::AppError;
use crate::repositories::traits::group_repo::GroupRepository;
use std::sync::Arc;
use uuid::Uuid;

pub fn require_non_empty(value: Option<String>, field: &str) -> Result<String, AppError> {
    let value = value.ok_or(AppError::BadRequest(format!("{field} empty")))?;

    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{field} empty")));
    }

    Ok(value)
}
