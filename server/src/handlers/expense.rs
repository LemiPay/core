use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::expense::Expense;
use crate::security::auth_extractor::AuthUser;
use axum::Json;
use axum::extract::{Path, State};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ParticipantInput {
    pub user_id: Uuid,
    pub amount: BigDecimal,
}

#[derive(Deserialize)]
pub struct CreateExpenseRequest {
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub description: Option<String>,
    pub participants: Vec<ParticipantInput>,
}

#[derive(Deserialize)]
pub struct UpdateExpenseRequest {
    pub currency_id: Option<Uuid>,
    pub amount: Option<BigDecimal>,
    pub description: Option<String>,
    pub participants: Option<Vec<ParticipantInput>>,
}

// Create: cualquier miembro del grupo puede cargar una expense
pub async fn create_expense(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    let result = state
        .expense_service
        .create_expense(user.user_id, group_id, payload)?;
    Ok(Json(result))
}

// Read: listar todas las expenses de un grupo
pub async fn get_expenses(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<Expense>>, AppError> {
    let result = state.expense_service.list_by_group(group_id)?;
    Ok(Json(result))
}

// Update (owner): el creador modifica los datos cargados
pub async fn update_expense(
    State(state): State<SharedState>,
    Path(expense_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    let result = state
        .expense_service
        .update_as_owner(user.user_id, expense_id, payload)?;
    Ok(Json(result))
}

// Update (admin): un admin del grupo modifica cualquier expense del grupo
pub async fn admin_update_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    let result = state
        .expense_service
        .update_as_admin(group_id, expense_id, payload)?;
    Ok(Json(result))
}

// Delete (owner): borrado lógico de la expense por su creador
pub async fn delete_expense(
    State(state): State<SharedState>,
    Path(expense_id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<Expense>, AppError> {
    let result = state
        .expense_service
        .delete_as_owner(user.user_id, expense_id)?;
    Ok(Json(result))
}

// Delete (admin): borrado lógico de cualquier expense del grupo
pub async fn admin_delete_expense(
    State(state): State<SharedState>,
    Path((group_id, expense_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Expense>, AppError> {
    let result = state
        .expense_service
        .delete_as_admin(group_id, expense_id)?;
    Ok(Json(result))
}
