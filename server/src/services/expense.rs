use std::collections::HashSet;
use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::handlers::expense::{CreateExpenseRequest, ParticipantInput, UpdateExpenseRequest};
use crate::models::expense::{Expense, ExpenseUpdate, MyExpenseStatus, NewExpense};
use crate::repositories::traits::expense_repo::ExpenseRepository;

#[derive(Clone)]
pub struct ExpenseService {
    expense_repo: Arc<dyn ExpenseRepository>,
}

impl ExpenseService {
    pub fn new(expense_repo: Arc<dyn ExpenseRepository>) -> Self {
        Self { expense_repo }
    }

    /// # Create expense
    /// El llamador debe pertenecer al grupo (chequeado por `is_in_group_middleware`).
    ///
    /// ### Errors
    /// - [`AppError::BadRequest`] si el monto es <= 0, la lista de participantes
    ///   está vacía o tiene duplicados.
    /// - [`AppError::Db`] ante cualquier error de base.
    pub fn create_expense(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        CreateExpenseRequest {
            currency_id,
            amount,
            description,
            participants,
        }: CreateExpenseRequest,
    ) -> Result<Expense, AppError> {
        Self::validate_amount(&amount)?;
        Self::validate_participants(&participants)?;
        let amount_per_participant = Self::split_amount_equally(&amount, participants.len())?;

        let new_expense = NewExpense {
            user_id,
            group_id,
            currency_id,
            amount,
            description,
        };

        let participants: Vec<(Uuid, BigDecimal)> = participants
            .into_iter()
            .map(|p| (p.user_id, amount_per_participant.clone()))
            .collect();

        let result = self.expense_repo.create(new_expense, participants)?;
        Ok(result)
    }

    /// # List expenses de un grupo
    /// El llamador debe pertenecer al grupo (chequeado por middleware).
    pub fn list_by_group(&self, group_id: Uuid) -> Result<Vec<Expense>, AppError> {
        let result = self.expense_repo.find_by_group(group_id)?;
        Ok(result)
    }

    /// # Update (owner)
    /// Sólo el creador de la expense puede modificarla.
    ///
    /// ### Errors
    /// - [`AppError::NotFound`] si no existe la expense.
    /// - [`AppError::Forbidden`] si el llamador no es el creador.
    /// - [`AppError::BadRequest`] si la expense ya está borrada lógicamente
    ///   o los datos son inválidos.
    pub fn update_as_owner(
        &self,
        user_id: Uuid,
        expense_id: Uuid,
        payload: UpdateExpenseRequest,
    ) -> Result<Expense, AppError> {
        let expense = self.get_active(expense_id)?;

        if expense.user_id != user_id {
            return Err(AppError::Forbidden(
                "Solo el creador o el admin pueden editar".into(),
            ));
        }

        self.apply_update(&expense, payload)
    }

    /// # Update (admin)
    /// Un admin del grupo puede modificar cualquier expense del grupo.
    /// `is_group_admin_middleware` ya validó el rol; acá chequeamos además
    /// que la expense efectivamente pertenezca al grupo indicado.
    pub fn update_as_admin(
        &self,
        group_id: Uuid,
        expense_id: Uuid,
        payload: UpdateExpenseRequest,
    ) -> Result<Expense, AppError> {
        let expense = self.get_active(expense_id)?;

        //aca en vez de tirar forbidden y decir que esa expense no pertenece a este grupo
        //tiro not found asi no pueden saber si esa expense existe o no
        //porque es de otro grupo
        if expense.group_id != group_id {
            return Err(AppError::NotFound);
        }

        self.apply_update(&expense, payload)
    }

    /// # Delete (owner)
    /// Borrado lógico por parte del creador.
    pub fn delete_as_owner(&self, user_id: Uuid, expense_id: Uuid) -> Result<Expense, AppError> {
        let expense = self.get_active(expense_id)?;

        if expense.user_id != user_id {
            return Err(AppError::Forbidden(
                "Solo el creador o el admin pueden editar".into(),
            ));
        }

        let result = self.expense_repo.soft_delete(expense_id)?;
        Ok(result)
    }

    /// # Delete (admin)
    /// Borrado lógico por parte de un admin del grupo.
    pub fn delete_as_admin(&self, group_id: Uuid, expense_id: Uuid) -> Result<Expense, AppError> {
        let expense = self.get_active(expense_id)?;

        //misma razon del not found de mas arriba
        if expense.group_id != group_id {
            return Err(AppError::NotFound);
        }

        let result = self.expense_repo.soft_delete(expense_id)?;
        Ok(result)
    }

    // ---------- helpers ----------

    fn get_active(&self, expense_id: Uuid) -> Result<Expense, AppError> {
        let expense = self
            .expense_repo
            .find_by_id(expense_id)?
            .ok_or(AppError::NotFound)?;

        if expense.status == MyExpenseStatus::Deleted {
            return Err(AppError::BadRequest("Expense is already deleted".into()));
        }

        Ok(expense)
    }

    fn apply_update(
        &self,
        expense: &Expense,
        UpdateExpenseRequest {
            currency_id,
            amount,
            description,
            participants,
        }: UpdateExpenseRequest,
    ) -> Result<Expense, AppError> {
        if currency_id.is_none()
            && amount.is_none()
            && description.is_none()
            && participants.is_none()
        {
            return Err(AppError::BadRequest("No fields to update".into()));
        }

        // Monto efectivo que tendrá la expense tras el update.
        let effective_amount = amount.clone().unwrap_or_else(|| expense.amount.clone());
        Self::validate_amount(&effective_amount)?;

        // Si vienen participantes, deben ser coherentes con el monto efectivo.
        let participants: Option<Vec<(Uuid, BigDecimal)>> = match participants {
            Some(ps) => {
                Self::validate_participants(&ps)?;
                let amount_per_participant =
                    Self::split_amount_equally(&effective_amount, ps.len())?;

                let mapped: Vec<(Uuid, BigDecimal)> = ps
                    .into_iter()
                    .map(|p| (p.user_id, amount_per_participant.clone()))
                    .collect();

                Some(mapped)
            }
            None => None,
        };

        let update = ExpenseUpdate {
            currency_id,
            amount,
            description,
        };

        let result = self
            .expense_repo
            .update(expense.expense_id, update, participants)?;
        Ok(result)
    }

    fn validate_amount(amount: &BigDecimal) -> Result<(), AppError> {
        if amount <= &BigDecimal::zero() {
            return Err(AppError::BadRequest("Amount must be greater than 0".into()));
        }
        Ok(())
    }

    fn validate_participants(participants: &[ParticipantInput]) -> Result<(), AppError> {
        if participants.is_empty() {
            return Err(AppError::BadRequest(
                "Expense must have at least one participant".into(),
            ));
        }

        let mut seen = HashSet::new();

        for p in participants {
            if !seen.insert(p.user_id) {
                return Err(AppError::BadRequest(
                    "Duplicated participant in expense".into(),
                ));
            }
        }

        Ok(())
    }

    fn split_amount_equally(
        total: &BigDecimal,
        participants_len: usize,
    ) -> Result<BigDecimal, AppError> {
        let participants_count = i64::try_from(participants_len)
            .map_err(|_| AppError::BadRequest("Too many participants".into()))?;
        let divisor = BigDecimal::from(participants_count);
        Ok(total / divisor)
    }
}
