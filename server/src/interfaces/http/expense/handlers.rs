use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    application::expense::dto::UpdateExpenseInput,
    interfaces::http::{
        auth::extractor::AuthUser,
        error::AppError,
        expense::dto::{CreateExpenseRequest, ExpenseResponse, UpdateExpenseRequest},
    },
    setup::state::SharedState,
};

pub async fn create_expense(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let item = state
        .expense_service
        .create_expense(
            user.user_id.0,
            group_id,
            payload.currency_id,
            payload.amount,
            payload.description,
            payload
                .participants
                .into_iter()
                .map(|p| p.user_id)
                .collect(),
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn get_expenses(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<ExpenseResponse>>, AppError> {
    let items = state
        .expense_service
        .list_by_group(group_id)
        .map_err(AppError::from)?;
    Ok(Json(items.into_iter().map(Into::into).collect()))
}

pub async fn update_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let item = state
        .expense_service
        .update_as_owner(
            user.user_id.0,
            expense_id,
            UpdateExpenseInput {
                currency_id: payload.currency_id,
                amount: payload.amount,
                description: payload.description,
                participants: payload
                    .participants
                    .map(|participants| participants.into_iter().map(|p| p.user_id).collect()),
            },
        )
        .map_err(AppError::from)?;

    if item.group_id != group_id {
        return Err(AppError::NotFound);
    }

    Ok(Json(item.into()))
}

pub async fn admin_update_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let item = state
        .expense_service
        .update_as_admin(
            group_id,
            expense_id,
            UpdateExpenseInput {
                currency_id: payload.currency_id,
                amount: payload.amount,
                description: payload.description,
                participants: payload
                    .participants
                    .map(|participants| participants.into_iter().map(|p| p.user_id).collect()),
            },
        )
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}

pub async fn delete_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
) -> Result<Json<ExpenseResponse>, AppError> {
    let item = state
        .expense_service
        .delete_as_owner(user.user_id.0, expense_id)
        .map_err(AppError::from)?;

    if item.group_id != group_id {
        return Err(AppError::NotFound);
    }

    Ok(Json(item.into()))
}

pub async fn admin_delete_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ExpenseResponse>, AppError> {
    let item = state
        .expense_service
        .delete_as_admin(group_id, expense_id)
        .map_err(AppError::from)?;
    Ok(Json(item.into()))
}
