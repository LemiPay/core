use crate::errors::app_error::AppError;
use uuid::Uuid;

pub fn require_non_empty(value: Option<String>, field: &str) -> Result<String, AppError> {
    let value = value.ok_or(AppError::BadRequest(format!("{field} empty")))?;

    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{field} empty")));
    }

    Ok(value)
}

pub fn require_non_empty_uuid(value: Option<Uuid>, field: &str) -> Result<Uuid, AppError> {
    let value = value.ok_or(AppError::BadRequest(format!("{field} empty")))?;

    if value.is_nil() {
        return Err(AppError::BadRequest(format!("{field} empty")));
    }

    Ok(value)
}
