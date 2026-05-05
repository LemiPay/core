use std::sync::Arc;

use bigdecimal::Zero;
use uuid::Uuid;

use crate::{
    application::{
        balances::{
            dto::{GroupBalancesDetails, UserBalanceDetails},
            error::BalancesError,
        },
        expense::{dto::ExpenseDetails, traits::repository::ExpenseRepository},
        group::traits::repository::GroupRepository,
        treasury::{dto::TransactionDetails, traits::transaction_repo::TransactionRepository},
    },
    domain::{
        balances::core,
        expense::{Expense, ExpenseId},
        group::GroupId,
        treasury::{CurrencyId, Money, Transaction, TransactionId},
        user::UserId,
    },
};

#[derive(Clone)]
pub struct BalancesService {
    pub transaction_repo: Arc<dyn TransactionRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub expense_repo: Arc<dyn ExpenseRepository>,
}

impl BalancesService {
    pub fn get_balances(&self, group_id: Uuid) -> Result<GroupBalancesDetails, BalancesError> {
        let historic_members = self
            .group_repo
            .get_historic_group_members(GroupId(group_id))
            .map_err(BalancesError::from)?;
        let user_ids: Vec<UserId> = historic_members.iter().map(|m| UserId(m.user_id)).collect();

        let transactions = self
            .transaction_repo
            .list_by_group(GroupId(group_id))
            .map_err(BalancesError::from)?;
        let expenses = self
            .expense_repo
            .find_by_group(group_id)
            .map_err(BalancesError::from)?;

        let balances_map = core(
            user_ids,
            transactions.into_iter().map(to_domain_tx).collect(),
            expenses.into_iter().map(to_domain_expense).collect(),
        )?;

        let balances = historic_members
            .iter()
            .map(|member| UserBalanceDetails {
                user_id: member.user_id,
                user_name: member.name.clone(),
                balance: balances_map
                    .get_user_balance(&UserId(member.user_id))
                    .cloned()
                    .unwrap_or_else(bigdecimal::BigDecimal::zero),
            })
            .collect();

        Ok(GroupBalancesDetails {
            group_balance: balances_map.get_group_balance().clone(),
            balances,
        })
    }
}

fn to_domain_tx(details: TransactionDetails) -> Transaction {
    Transaction {
        id: TransactionId(details.id),
        tx_hash: details.tx_hash,
        amount: Money {
            amount: details.amount,
            currency: CurrencyId(details.currency_id),
        },
        user_id: UserId(details.user_id),
        group_id: GroupId(details.group_id),
        address: details.address,
        description: details.description,
        tx_type: details.tx_type,
    }
}

fn to_domain_expense(details: ExpenseDetails) -> Expense {
    Expense::rehydrate(
        ExpenseId(details.expense_id),
        UserId(details.user_id),
        GroupId(details.group_id),
        CurrencyId(details.currency_id),
        details.description,
        details.amount,
        details.status,
        details.created_at,
        details.updated_at,
    )
}
