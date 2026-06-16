use std::{str::FromStr, sync::Arc};

use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use chrono::Utc;
use rand::Rng;
use uuid::Uuid;

use crate::application::balances::BalancesService;
use crate::application::balances::dto::UserBalanceDetails;
use crate::application::{
    common::repo_error::RepoError,
    group::traits::repository::GroupRepository,
    investment::{
        dto::{
            ActiveInvestmentDto, InvestmentDetails, InvestmentProposalDetails,
            InvestmentStrategyDto, PulseResult, SnapshotDto,
        },
        error::InvestmentError,
        traits::repository::InvestmentRepository,
    },
    treasury::traits::group_wallet_repo::GroupWalletRepository,
};
use crate::domain::group::GroupPolicy;
use crate::domain::investment::member::NewInvestmentMember;
use crate::domain::user::UserId;
use crate::domain::{
    group::GroupId,
    investment::{Investment, InvestmentPolicy},
    treasury::{CurrencyId, Money, TreasuryError, TreasuryPolicy},
};
#[derive(Clone)]
pub struct InvestmentService {
    pub investment_repo: Arc<dyn InvestmentRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub balances_service: BalancesService,
}

impl InvestmentService {
    fn parse_amount(raw: &str) -> Result<BigDecimal, InvestmentError> {
        BigDecimal::from_str(raw).map_err(|_| InvestmentError::InvalidAmount)
    }

    fn map_repo<T>(result: Result<T, RepoError>) -> Result<T, InvestmentError> {
        result.map_err(InvestmentError::from)
    }

    // ── Strategies ──

    pub fn list_strategies(&self) -> Result<Vec<InvestmentStrategyDto>, InvestmentError> {
        Self::map_repo(self.investment_repo.list_strategies())
    }

    // ── Investment Proposals ──

    pub fn list_approved_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<InvestmentProposalDetails>, InvestmentError> {
        Self::map_repo(self.investment_repo.list_approved_proposals(group_id))
    }

    pub fn create_investment_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        amount: String,
        strategy_id: Uuid,
        currency_id: Uuid,
    ) -> Result<InvestmentProposalDetails, InvestmentError> {
        let group = Self::map_repo(self.group_repo.find_by_id(GroupId(group_id)))?
            .ok_or(InvestmentError::NotFound)?;
        GroupPolicy::ensure_active(&group).map_err(|_| InvestmentError::GroupNotActive)?;
        let amount = Self::parse_amount(&amount)?;
        InvestmentPolicy::ensure_positive_amount(&amount)?;

        Self::map_repo(self.investment_repo.find_strategy(strategy_id))?
            .ok_or(InvestmentError::StrategyNotFound)?;

        let wallet = Self::map_repo(
            self.group_wallet_repo
                .find_by_group_and_currency(GroupId(group_id), CurrencyId(currency_id)),
        )?
        .ok_or(InvestmentError::GroupWalletNotFound)?;

        let amount_money = Money::positive(amount.clone(), CurrencyId(currency_id))
            .map_err(|_| InvestmentError::InvalidAmount)?;

        let has_enough = wallet
            .balance
            .has_enough(&amount_money)
            .map_err(|_| InvestmentError::Internal)?;

        if !has_enough {
            return Err(InvestmentError::InsufficientGroupFunds);
        }

        Self::map_repo(self.investment_repo.create_investment_proposal(
            created_by,
            group_id,
            amount,
            strategy_id,
            currency_id,
        ))
    }

    // ── Execute Investment ──

    pub fn execute_investment_proposal(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        proposal_id: Uuid,
    ) -> Result<InvestmentDetails, InvestmentError> {
        let group = Self::map_repo(self.group_repo.find_by_id(GroupId(group_id)))?
            .ok_or(InvestmentError::NotFound)?;
        GroupPolicy::ensure_active(&group).map_err(|_| InvestmentError::GroupNotActive)?;

        let proposal = Self::map_repo(self.investment_repo.find_investment_proposal(proposal_id))?
            .ok_or(InvestmentError::ProposalNotFound)?;

        if proposal.group_id != group_id {
            return Err(InvestmentError::NotFound);
        }

        let strategy = Self::map_repo(self.investment_repo.find_strategy(proposal.strategy_id))?
            .ok_or(InvestmentError::StrategyNotFound)?;

        let wallet = Self::map_repo(
            self.group_wallet_repo
                .find_by_group_and_currency(GroupId(group_id), CurrencyId(proposal.currency_id)),
        )?
        .ok_or(InvestmentError::GroupWalletNotFound)?;

        let amount_money =
            Money::positive(proposal.amount.clone(), CurrencyId(proposal.currency_id))
                .map_err(|_| InvestmentError::InvalidAmount)?;

        TreasuryPolicy::ensure_group_can_cover(&wallet, &amount_money).map_err(|e| match e {
            TreasuryError::InsufficientFunds => InvestmentError::InsufficientGroupFunds,
            _ => InvestmentError::Internal,
        })?;

        let matures_at =
            Investment::calculate_matures_at(Utc::now().naive_utc(), strategy.duration_days);

        let balances = self
            .balances_service
            .get_balances(GroupId(group_id))
            .map_err(|_| InvestmentError::Internal)?;

        let positive_balances: Vec<UserBalanceDetails> = balances
            .balances
            .into_iter()
            .filter(|b| b.balance > BigDecimal::zero())
            .collect();
        let sum = positive_balances
            .iter()
            .map(|b| &b.balance)
            .sum::<BigDecimal>();

        let hundred = BigDecimal::from(100);
        let mut participants: Vec<NewInvestmentMember> = positive_balances
            .iter()
            .map(|b| {
                let pct = (&b.balance / &sum) * &hundred;
                let invested_amount = &proposal.amount * &pct / &hundred;
                NewInvestmentMember {
                    user_id: UserId(b.user_id),
                    balance_at_investment: b.balance.clone(),
                    participation_pct: pct,
                    invested_amount,
                    returned_amount: None,
                    withdrawn_at: None,
                }
            })
            .collect();
        let max_idx = participants
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.balance_at_investment.cmp(&b.balance_at_investment))
            .map(|(idx, _)| idx);
        if let Some(max_idx) = max_idx {
            let others_sum: BigDecimal = participants
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != max_idx)
                .map(|(_, p)| &p.participation_pct)
                .sum();
            participants[max_idx].participation_pct = &hundred - others_sum;
            participants[max_idx].invested_amount =
                &proposal.amount * &participants[max_idx].participation_pct / &hundred;
        }

        Self::map_repo(self.investment_repo.execute_investment(
            proposal_id,
            group_id,
            user_id,
            proposal.amount,
            proposal.strategy_id,
            proposal.currency_id,
            matures_at,
            participants,
        ))
    }

    // ── Withdraw Investment ──

    pub fn withdraw_investment(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        investment_id: Uuid,
    ) -> Result<InvestmentDetails, InvestmentError> {
        let stored = Self::map_repo(self.investment_repo.find_investment(investment_id))?
            .ok_or(InvestmentError::NotFound)?;

        if stored.group_id != group_id {
            return Err(InvestmentError::NotFound);
        }

        let group = Self::map_repo(self.group_repo.find_by_id(GroupId(group_id)))?
            .ok_or(InvestmentError::NotFound)?;
        GroupPolicy::ensure_active(&group).map_err(|_| InvestmentError::GroupNotActive)?;

        let domain = Investment::rehydrate(
            crate::domain::investment::InvestmentId(stored.id),
            GroupId(stored.group_id),
            crate::domain::governance::ProposalId(stored.proposal_id),
            crate::domain::investment::InvestmentStrategyId(stored.strategy_id),
            CurrencyId(stored.currency_id),
            stored.amount.clone(),
            stored.current_value.clone(),
            stored.actual_return.clone(),
            stored.status,
            stored.started_at,
            stored.matures_at,
            stored.created_at,
            stored.updated_at,
        );

        InvestmentPolicy::ensure_can_withdraw(&domain)?;

        Self::map_repo(
            self.investment_repo
                .withdraw_investment(investment_id, group_id, user_id),
        )
    }

    // ── List ──

    pub fn list_group_investments(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<InvestmentDetails>, InvestmentError> {
        Self::map_repo(self.investment_repo.list_group_investments(group_id))
    }

    // ── Snapshots ──

    pub fn list_snapshots(
        &self,
        investment_id: Uuid,
        user_id: UserId,
    ) -> Result<Vec<SnapshotDto>, InvestmentError> {
        let investment = self
            .investment_repo
            .find_investment(investment_id)?
            .ok_or(InvestmentError::NotFound)?;
        let group_id = GroupId(investment.group_id);
        if !self
            .group_repo
            .is_member(user_id, group_id)
            .map_err(InvestmentError::from)?
        {
            return Err(InvestmentError::NotGroupMember);
        }
        Self::map_repo(self.investment_repo.list_snapshots(investment_id))
    }

    // ── Pulse ──

    pub fn process_pulse(&self) -> Result<PulseResult, String> {
        use bigdecimal::BigDecimal;

        let active: Vec<ActiveInvestmentDto> = self
            .investment_repo
            .list_active_with_strategy()
            .map_err(|e| format!("Failed to query active investments: {:?}", e))?;

        if active.is_empty() {
            return Ok(PulseResult {
                updated: 0,
                matured: 0,
                matured_group_ids: Vec::new(),
            });
        }

        let mut rng = rand::rngs::OsRng;
        let now = Utc::now().naive_utc();
        let mut updated = 0;
        let mut matured = 0;
        let mut matured_group_ids = Vec::new();

        for inv in &active {
            let snapshot_count = self
                .investment_repo
                .count_snapshots(inv.id)
                .map_err(|e| format!("Failed to count snapshots: {:?}", e))?;

            let accrued_days = snapshot_count + 1;
            let is_last_day = accrued_days >= inv.duration_days as i64;

            let days = BigDecimal::from_i64(accrued_days).unwrap();
            let duration = BigDecimal::from_i32(inv.duration_days).unwrap();
            let hundred = BigDecimal::from(100);

            let linear_value = inv.amount.clone()
                * (BigDecimal::from(1)
                    + &inv.expected_return_percentage / &hundred * &days / &duration);

            let noise_pct = daily_noise_range(&inv.risk_level);
            let noise_factor =
                BigDecimal::from_f64(1.0 + rng.gen_range(-noise_pct..=noise_pct)).unwrap();
            let current_value = linear_value * noise_factor;

            if is_last_day {
                let variation_pct = risk_variation_range(&inv.risk_level);
                let variation: f64 = rng.gen_range(-variation_pct..=variation_pct);

                let return_portion = &current_value - &inv.amount;
                let varied_return = if return_portion.is_zero() {
                    BigDecimal::zero()
                } else {
                    return_portion * BigDecimal::from_f64(1.0 + variation).unwrap()
                };

                let final_value = &inv.amount + &varied_return;

                self.investment_repo
                    .mature_investment(inv.id, final_value.clone(), varied_return.clone(), now)
                    .map_err(|e| format!("Failed to mature investment {}: {:?}", inv.id, e))?;

                self.investment_repo
                    .insert_snapshot(inv.id, final_value, now)
                    .map_err(|e| format!("Failed to insert snapshot: {:?}", e))?;

                matured += 1;
                matured_group_ids.push(inv.group_id);
            } else {
                self.investment_repo
                    .update_current_value(inv.id, current_value.clone(), now)
                    .map_err(|e| {
                        format!("Failed to update current_value for {}: {:?}", inv.id, e)
                    })?;

                self.investment_repo
                    .insert_snapshot(inv.id, current_value, now)
                    .map_err(|e| format!("Failed to insert snapshot: {:?}", e))?;
            }

            updated += 1;
        }

        matured_group_ids.sort_unstable();
        matured_group_ids.dedup();

        Ok(PulseResult {
            updated,
            matured,
            matured_group_ids,
        })
    }
}

fn daily_noise_range(risk_level: &str) -> f64 {
    match risk_level {
        "low" => 0.005,
        "medium" => 0.01,
        "high" => 0.02,
        _ => 0.0,
    }
}

fn risk_variation_range(risk_level: &str) -> f64 {
    match risk_level {
        "low" => 0.01,
        "medium" => 0.05,
        "high" => 0.10,
        _ => 0.0,
    }
}
