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
            InvestmentStrategyDto, NewHolding, PulseResult, SnapshotDto,
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
    investment::{Investment, InvestmentPolicy, ValuationMode},
    treasury::{CurrencyId, Money, TreasuryError, TreasuryPolicy},
};
use crate::infrastructure::market_data::{AssetPriceRef, PriceOracle};

#[derive(Clone)]
pub struct InvestmentService {
    pub investment_repo: Arc<dyn InvestmentRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub balances_service: BalancesService,
    pub price_oracle: Arc<dyn PriceOracle>,
}

impl InvestmentService {
    fn parse_amount(raw: &str) -> Result<BigDecimal, InvestmentError> {
        BigDecimal::from_str(raw).map_err(|_| InvestmentError::InvalidAmount)
    }

    fn map_repo<T>(result: Result<T, RepoError>) -> Result<T, InvestmentError> {
        result.map_err(InvestmentError::from)
    }

    fn to_price_refs(
        assets: &[crate::application::investment::dto::AssetPriceDto],
    ) -> Vec<AssetPriceRef> {
        assets
            .iter()
            .map(|a| AssetPriceRef {
                id: a.id,
                symbol: a.symbol.clone(),
                price_provider: a.price_provider.clone(),
                external_id: a.external_id.clone(),
            })
            .collect()
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

    pub async fn execute_investment_proposal(
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

        // Build holdings for mark-to-market strategies
        let mut holdings: Vec<NewHolding> = Vec::new();
        if ValuationMode::parse(&strategy.valuation_mode) == ValuationMode::MarkToMarket {
            if strategy.allocations.is_empty() {
                return Err(InvestmentError::StrategyNotFound);
            }
            let price_assets = Self::map_repo(
                self.investment_repo
                    .list_strategy_assets_for_pricing(strategy.id),
            )?;
            let refs = Self::to_price_refs(&price_assets);
            let prices = self.price_oracle.get_usd_prices(&refs).await.map_err(|e| {
                eprintln!("execute: price oracle error: {e}");
                InvestmentError::PriceUnavailable
            })?;

            let ten_thousand = BigDecimal::from(10_000);
            for alloc in &strategy.allocations {
                let price = prices.get(&alloc.asset_id).ok_or_else(|| {
                    eprintln!(
                        "execute: missing price for asset_id={} in strategy {}",
                        alloc.asset_id, strategy.name
                    );
                    InvestmentError::PriceUnavailable
                })?;
                if price <= &BigDecimal::zero() {
                    eprintln!(
                        "execute: non-positive price for asset_id={} price={}",
                        alloc.asset_id, price
                    );
                    return Err(InvestmentError::PriceUnavailable);
                }
                let notional =
                    &proposal.amount * BigDecimal::from(alloc.weight_bps) / &ten_thousand;
                let units = &notional / price;
                holdings.push(NewHolding {
                    asset_id: alloc.asset_id,
                    units,
                    weight_bps_at_entry: alloc.weight_bps,
                    cost_basis_usd: notional,
                });
            }
        }

        let mut details = Self::map_repo(self.investment_repo.execute_investment(
            proposal_id,
            group_id,
            user_id,
            proposal.amount,
            proposal.strategy_id,
            proposal.currency_id,
            matures_at,
            participants,
            holdings,
        ))?;
        self.enrich_holdings_with_current_prices(&mut details.holdings)
            .await;
        Ok(details)
    }

    // ── Withdraw / Ragequit ──

    pub async fn withdraw_investment(
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

        match stored.status {
            crate::domain::investment::InvestmentStatus::Matured => {
                InvestmentPolicy::ensure_can_withdraw(&domain)?;
                // Prefer actual_return + amount if set; else current_value
                let payout = if let Some(ret) = &stored.actual_return {
                    &stored.amount + ret
                } else {
                    stored.current_value.clone()
                };
                let actual_return = &payout - &stored.amount;
                Self::map_repo(self.investment_repo.withdraw_investment(
                    investment_id,
                    group_id,
                    user_id,
                    payout,
                    actual_return,
                    "maturity",
                    BigDecimal::zero(),
                ))
            }
            crate::domain::investment::InvestmentStatus::Active => {
                InvestmentPolicy::ensure_can_ragequit(&domain)?;
                // Re-price MTM if possible before exit
                let nav = self
                    .reprice_nav(investment_id, &stored.valuation_mode)
                    .await
                    .unwrap_or(stored.current_value.clone());
                let (payout, fee) =
                    InvestmentPolicy::ragequit_payout(&nav, stored.ragequit_fee_bps);
                let actual_return = &payout - &stored.amount;
                Self::map_repo(self.investment_repo.withdraw_investment(
                    investment_id,
                    group_id,
                    user_id,
                    payout,
                    actual_return,
                    "ragequit",
                    fee,
                ))
            }
            crate::domain::investment::InvestmentStatus::Withdrawn => {
                Err(InvestmentError::AlreadyWithdrawn)
            }
        }
    }

    async fn reprice_nav(
        &self,
        investment_id: Uuid,
        valuation_mode: &str,
    ) -> Result<BigDecimal, InvestmentError> {
        if ValuationMode::parse(valuation_mode) != ValuationMode::MarkToMarket {
            let inv = Self::map_repo(self.investment_repo.find_investment(investment_id))?
                .ok_or(InvestmentError::NotFound)?;
            return Ok(inv.current_value);
        }
        let holdings = Self::map_repo(
            self.investment_repo
                .list_holdings_for_pricing(investment_id),
        )?;
        if holdings.is_empty() {
            let inv = Self::map_repo(self.investment_repo.find_investment(investment_id))?
                .ok_or(InvestmentError::NotFound)?;
            return Ok(inv.current_value);
        }
        let refs: Vec<AssetPriceRef> = holdings
            .iter()
            .map(|(a, _)| AssetPriceRef {
                id: a.id,
                symbol: a.symbol.clone(),
                price_provider: a.price_provider.clone(),
                external_id: a.external_id.clone(),
            })
            .collect();
        let prices = self
            .price_oracle
            .get_usd_prices(&refs)
            .await
            .map_err(|_| InvestmentError::PriceUnavailable)?;
        let mut nav = BigDecimal::zero();
        for (asset, units) in &holdings {
            let price = prices
                .get(&asset.id)
                .ok_or(InvestmentError::PriceUnavailable)?;
            nav += units * price;
        }
        Ok(nav)
    }

    // ── List ──

    pub async fn list_group_investments(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<InvestmentDetails>, InvestmentError> {
        let mut items = Self::map_repo(self.investment_repo.list_group_investments(group_id))?;
        for inv in &mut items {
            self.enrich_holdings_with_current_prices(&mut inv.holdings)
                .await;
        }
        Ok(items)
    }

    /// Fills current_price_usd / current_value_usd on holdings via the price oracle.
    async fn enrich_holdings_with_current_prices(
        &self,
        holdings: &mut [crate::application::investment::dto::HoldingDto],
    ) {
        use crate::infrastructure::market_data::MockPriceOracle;

        if holdings.is_empty() {
            return;
        }
        let refs: Vec<AssetPriceRef> = holdings
            .iter()
            .map(|h| AssetPriceRef {
                id: h.asset_id,
                symbol: h.symbol.clone(),
                price_provider: h.price_provider.clone(),
                external_id: h.external_id.clone(),
            })
            .collect();

        let mut prices = match self.price_oracle.get_usd_prices(&refs).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("price oracle enrich failed ({e}); falling back to mock marks");
                MockPriceOracle::new().prices_for(&refs)
            }
        };

        // Ensure every holding has a mark (never leave UI empty).
        for h in holdings.iter_mut() {
            if !prices.contains_key(&h.asset_id) {
                if let Some(entry) = h.entry_price_usd.clone() {
                    prices.insert(h.asset_id, entry);
                }
            }
            if let Some(price) = prices.get(&h.asset_id) {
                h.current_price_usd = Some(price.clone());
                h.current_value_usd = Some(&h.units * price);
            } else if let Some(entry) = &h.entry_price_usd {
                // Last resort: freeze at entry
                h.current_price_usd = Some(entry.clone());
                h.current_value_usd = Some(h.cost_basis_usd.clone());
            }
        }
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

    pub async fn process_pulse(&self) -> Result<PulseResult, String> {
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

            let current_value =
                if ValuationMode::parse(&inv.valuation_mode) == ValuationMode::MarkToMarket {
                    match self.compute_mtm_nav(inv.id).await {
                        Ok(nav) => nav,
                        Err(e) => {
                            eprintln!("MTM pulse skip for {}: {}", inv.id, e);
                            continue;
                        }
                    }
                } else {
                    self.compute_simulated_value(inv, accrued_days, &mut rng, is_last_day)
                };

            if is_last_day {
                let final_value =
                    if ValuationMode::parse(&inv.valuation_mode) == ValuationMode::MarkToMarket {
                        current_value.clone()
                    } else {
                        // apply final variation already baked into compute for last day
                        current_value.clone()
                    };
                let varied_return = &final_value - &inv.amount;

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

    async fn compute_mtm_nav(&self, investment_id: Uuid) -> Result<BigDecimal, String> {
        let holdings = self
            .investment_repo
            .list_holdings_for_pricing(investment_id)
            .map_err(|e| format!("{:?}", e))?;
        if holdings.is_empty() {
            return Err("no holdings".into());
        }
        let refs: Vec<AssetPriceRef> = holdings
            .iter()
            .map(|(a, _)| AssetPriceRef {
                id: a.id,
                symbol: a.symbol.clone(),
                price_provider: a.price_provider.clone(),
                external_id: a.external_id.clone(),
            })
            .collect();
        let prices = self
            .price_oracle
            .get_usd_prices(&refs)
            .await
            .map_err(|e| e.to_string())?;
        let mut nav = BigDecimal::zero();
        for (asset, units) in &holdings {
            let price = prices
                .get(&asset.id)
                .ok_or_else(|| format!("missing price {}", asset.symbol))?;
            nav += units * price;
        }
        Ok(nav)
    }

    fn compute_simulated_value(
        &self,
        inv: &ActiveInvestmentDto,
        accrued_days: i64,
        rng: &mut impl Rng,
        is_last_day: bool,
    ) -> BigDecimal {
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

            &inv.amount + &varied_return
        } else {
            current_value
        }
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
