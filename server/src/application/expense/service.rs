use std::{str::FromStr, sync::Arc};

use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{
    application::expense::{
        dto::{ExpenseDetails, ExpenseUpdate, NewExpense, UpdateExpenseInput},
        error::ExpenseError,
        traits::repository::ExpenseRepository,
    },
    domain::{
        expense::{Expense, ExpenseId, ExpensePolicy},
        group::GroupId,
        treasury::CurrencyId,
        user::UserId,
    },
};

#[derive(Clone)]
pub struct ExpenseService {
    pub expense_repo: Arc<dyn ExpenseRepository>,
}

impl ExpenseService {
    pub fn create_expense(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        currency_id: Uuid,
        amount: String,
        description: Option<String>,
        participants: Vec<Uuid>,
    ) -> Result<ExpenseDetails, ExpenseError> {
        let amount = parse_amount(&amount)?;
        ExpensePolicy::ensure_positive_amount(&amount)?;
        let participants = validate_and_split_participants(participants, &amount)?;

        self.expense_repo
            .create(
                NewExpense {
                    user_id,
                    group_id,
                    currency_id,
                    amount,
                    description,
                },
                participants,
            )
            .map_err(ExpenseError::from)
    }

    pub fn list_by_group(&self, group_id: Uuid) -> Result<Vec<ExpenseDetails>, ExpenseError> {
        self.expense_repo
            .find_by_group(group_id)
            .map_err(ExpenseError::from)
    }

    pub fn update_as_owner(
        &self,
        user_id: Uuid,
        expense_id: Uuid,
        payload: UpdateExpenseInput,
    ) -> Result<ExpenseDetails, ExpenseError> {
        let expense = self.get_active(expense_id)?;
        ExpensePolicy::ensure_owner(&expense, UserId(user_id))?;
        self.apply_update(&expense, payload)
    }

    pub fn update_as_admin(
        &self,
        group_id: Uuid,
        expense_id: Uuid,
        payload: UpdateExpenseInput,
    ) -> Result<ExpenseDetails, ExpenseError> {
        let expense = self.get_active(expense_id)?;
        ExpensePolicy::ensure_in_group(&expense, GroupId(group_id))?;
        self.apply_update(&expense, payload)
    }

    pub fn delete_as_owner(
        &self,
        user_id: Uuid,
        expense_id: Uuid,
    ) -> Result<ExpenseDetails, ExpenseError> {
        let expense = self.get_active(expense_id)?;
        ExpensePolicy::ensure_owner(&expense, UserId(user_id))?;
        self.expense_repo
            .soft_delete(expense_id)
            .map_err(ExpenseError::from)
    }

    pub fn delete_as_admin(
        &self,
        group_id: Uuid,
        expense_id: Uuid,
    ) -> Result<ExpenseDetails, ExpenseError> {
        let expense = self.get_active(expense_id)?;
        ExpensePolicy::ensure_in_group(&expense, GroupId(group_id))?;
        self.expense_repo
            .soft_delete(expense_id)
            .map_err(ExpenseError::from)
    }

    fn get_active(&self, expense_id: Uuid) -> Result<Expense, ExpenseError> {
        let stored = self
            .expense_repo
            .find_by_id(expense_id)
            .map_err(ExpenseError::from)?
            .ok_or(ExpenseError::NotFound)?;
        let expense = to_domain(&stored);
        ExpensePolicy::ensure_not_deleted(&expense)?;
        Ok(expense)
    }

    fn apply_update(
        &self,
        expense: &Expense,
        payload: UpdateExpenseInput,
    ) -> Result<ExpenseDetails, ExpenseError> {
        if payload.currency_id.is_none()
            && payload.amount.is_none()
            && payload.description.is_none()
            && payload.participants.is_none()
        {
            return Err(ExpenseError::NoFieldsToUpdate);
        }

        let effective_amount = match payload.amount.as_deref() {
            Some(raw) => parse_amount(raw)?,
            None => expense.amount.clone(),
        };
        ExpensePolicy::ensure_positive_amount(&effective_amount)?;

        let participants = match payload.participants {
            Some(participants) => Some(validate_and_split_participants(
                participants,
                &effective_amount,
            )?),
            None => None,
        };

        let update = ExpenseUpdate {
            currency_id: payload.currency_id,
            amount: payload.amount.as_deref().map(parse_amount).transpose()?,
            description: payload.description,
        };

        self.expense_repo
            .update(expense.id.0, update, participants)
            .map_err(ExpenseError::from)
    }
}

fn to_domain(stored: &ExpenseDetails) -> Expense {
    Expense::rehydrate(
        ExpenseId(stored.expense_id),
        UserId(stored.user_id),
        GroupId(stored.group_id),
        CurrencyId(stored.currency_id),
        stored.description.clone(),
        stored.amount.clone(),
        stored.status,
        stored.created_at,
        stored.updated_at,
    )
}

fn validate_and_split_participants(
    participants: Vec<Uuid>,
    total_amount: &BigDecimal,
) -> Result<Vec<(Uuid, BigDecimal)>, ExpenseError> {
    let participant_ids: Vec<UserId> = participants.into_iter().map(UserId).collect();
    ExpensePolicy::ensure_unique_non_empty_participants(&participant_ids)?;

    let amount_per_participant =
        ExpensePolicy::split_amount_equally(total_amount, participant_ids.len())?;

    Ok(participant_ids
        .into_iter()
        .map(|participant| (participant.0, amount_per_participant.clone()))
        .collect())
}

fn parse_amount(raw: &str) -> Result<BigDecimal, ExpenseError> {
    BigDecimal::from_str(raw).map_err(|_| ExpenseError::InvalidAmount)
}
