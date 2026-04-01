use crate::data::error::DbError;
use crate::errors::app_error::AppError;
use crate::models::proposal::Proposal;
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

pub(crate) fn check_proposal_exists(
    proposal: Result<Option<Proposal>, DbError>,
) -> Result<bool, DbError> {
    match proposal {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e), // Handle the error as needed
    }
}
