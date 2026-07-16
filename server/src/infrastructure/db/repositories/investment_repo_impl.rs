use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, prelude::*,
};
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    investment::dto::{
        ActiveInvestmentDto, AllocationDto, AssetPriceDto, HoldingDto, InvestmentDetails,
        InvestmentProposalDetails, InvestmentStrategyDto, NewHolding, SnapshotDto,
    },
    investment::traits::repository::InvestmentRepository,
};
use crate::domain::investment::InvestmentStatus;
use crate::domain::investment::member::NewInvestmentMember;
use crate::domain::treasury::TransactionType;
use crate::infrastructure::db::{
    models::{
        governance::{
            NewProposalModel, ProposalModel, ProposalStatusModel, ProposalStatusUpdateModel,
        },
        investment::{
            AssetModel, InvestmentHoldingModel, InvestmentMemberModel, InvestmentModel,
            InvestmentProposalModel, InvestmentStatusModel, InvestmentStrategyModel,
            NewInvestmentHoldingModel, NewInvestmentMemberModel, NewInvestmentModel,
            NewInvestmentProposalModel, StrategyAllocationModel,
        },
        treasury::{NewTransactionModel, TransactionTypeModel},
    },
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselInvestmentRepository {
    db: DbPool,
}

impl DieselInvestmentRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }

    /// CoinGecko asset page — id comes from `config/coingecko_tickers.toml`.
    fn price_source_url(_kind: &str, _provider: &str, symbol: &str, external_id: &str) -> String {
        crate::infrastructure::market_data::TickerMap::global().price_page_url(symbol, external_id)
    }

    fn load_allocations(
        conn: &mut DbConn,
        strategy_id: Uuid,
    ) -> Result<Vec<AllocationDto>, diesel::result::Error> {
        let rows = schema::strategy_allocation::table
            .filter(schema::strategy_allocation::strategy_id.eq(strategy_id))
            .inner_join(
                schema::asset::table
                    .on(schema::strategy_allocation::asset_id.eq(schema::asset::id)),
            )
            .select((
                StrategyAllocationModel::as_select(),
                AssetModel::as_select(),
            ))
            .load::<(StrategyAllocationModel, AssetModel)>(conn)?;
        Ok(rows
            .into_iter()
            .map(|(alloc, asset)| {
                let price_source_url = Self::price_source_url(
                    &asset.kind,
                    &asset.price_provider,
                    &asset.symbol,
                    &asset.external_id,
                );
                AllocationDto {
                    asset_id: asset.id,
                    symbol: asset.symbol,
                    name: asset.name,
                    kind: asset.kind,
                    weight_bps: alloc.weight_bps,
                    price_provider: asset.price_provider,
                    external_id: asset.external_id,
                    price_source_url,
                }
            })
            .collect())
    }

    fn load_holdings(
        conn: &mut DbConn,
        investment_id: Uuid,
    ) -> Result<Vec<HoldingDto>, diesel::result::Error> {
        let rows = schema::investment_holding::table
            .filter(schema::investment_holding::investment_id.eq(investment_id))
            .inner_join(
                schema::asset::table.on(schema::investment_holding::asset_id.eq(schema::asset::id)),
            )
            .select((InvestmentHoldingModel::as_select(), AssetModel::as_select()))
            .load::<(InvestmentHoldingModel, AssetModel)>(conn)?;
        Ok(rows
            .into_iter()
            .map(|(h, asset)| {
                let price_source_url = Self::price_source_url(
                    &asset.kind,
                    &asset.price_provider,
                    &asset.symbol,
                    &asset.external_id,
                );
                let entry_price_usd = if h.units > BigDecimal::from(0) {
                    Some(&h.cost_basis_usd / &h.units)
                } else {
                    None
                };
                HoldingDto {
                    asset_id: asset.id,
                    symbol: asset.symbol,
                    name: asset.name,
                    kind: asset.kind,
                    units: h.units,
                    weight_bps_at_entry: h.weight_bps_at_entry,
                    cost_basis_usd: h.cost_basis_usd,
                    price_provider: asset.price_provider,
                    external_id: asset.external_id,
                    price_source_url,
                    entry_price_usd,
                    current_price_usd: None,
                    current_value_usd: None,
                }
            })
            .collect())
    }

    fn strategy_to_dto(
        s: InvestmentStrategyModel,
        allocations: Vec<AllocationDto>,
    ) -> InvestmentStrategyDto {
        InvestmentStrategyDto {
            id: s.id,
            name: s.name,
            description: s.description,
            risk_level: s.risk_level,
            expected_return_percentage: s.expected_return_percentage,
            duration_days: s.duration_days,
            created_at: s.created_at,
            valuation_mode: s.valuation_mode,
            category: s.category,
            ragequit_fee_bps: s.ragequit_fee_bps,
            allocations,
        }
    }

    fn to_investment_details(
        inv: InvestmentModel,
        group_id: Uuid,
        strategy_id: Uuid,
        currency_id: Uuid,
        strategy: &InvestmentStrategyModel,
        holdings: Vec<HoldingDto>,
    ) -> InvestmentDetails {
        InvestmentDetails {
            id: inv.id,
            group_id,
            proposal_id: inv.proposal_id,
            strategy_id,
            currency_id,
            amount: inv.amount,
            current_value: inv.current_value,
            actual_return: inv.actual_return,
            status: inv.status.into(),
            started_at: inv.started_at,
            matures_at: inv.matures_at,
            created_at: inv.created_at,
            updated_at: inv.updated_at,
            strategy_name: strategy.name.clone(),
            risk_level: strategy.risk_level.clone(),
            expected_return_percentage: strategy.expected_return_percentage.clone(),
            valuation_mode: strategy.valuation_mode.clone(),
            category: strategy.category.clone(),
            ragequit_fee_bps: strategy.ragequit_fee_bps,
            exit_kind: inv.exit_kind,
            fee_amount: inv.fee_amount,
            holdings,
        }
    }
}

impl InvestmentRepository for DieselInvestmentRepository {
    fn list_strategies(&self) -> Result<Vec<InvestmentStrategyDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::investment_strategy::table
            .select(InvestmentStrategyModel::as_select())
            .order_by(schema::investment_strategy::created_at.asc())
            .load::<InvestmentStrategyModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        let mut out = Vec::with_capacity(rows.len());
        for s in rows {
            let allocations =
                Self::load_allocations(&mut conn, s.id).map_err(|_| RepoError::Query)?;
            out.push(Self::strategy_to_dto(s, allocations));
        }
        Ok(out)
    }

    fn find_strategy(&self, strategy_id: Uuid) -> Result<Option<InvestmentStrategyDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::investment_strategy::table
            .filter(schema::investment_strategy::id.eq(strategy_id))
            .select(InvestmentStrategyModel::as_select())
            .first::<InvestmentStrategyModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        match row {
            Some(s) => {
                let allocations =
                    Self::load_allocations(&mut conn, s.id).map_err(|_| RepoError::Query)?;
                Ok(Some(Self::strategy_to_dto(s, allocations)))
            }
            None => Ok(None),
        }
    }

    fn list_strategy_assets_for_pricing(
        &self,
        strategy_id: Uuid,
    ) -> Result<Vec<AssetPriceDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::strategy_allocation::table
            .filter(schema::strategy_allocation::strategy_id.eq(strategy_id))
            .inner_join(
                schema::asset::table
                    .on(schema::strategy_allocation::asset_id.eq(schema::asset::id)),
            )
            .select(AssetModel::as_select())
            .load::<AssetModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|a| AssetPriceDto {
                id: a.id,
                symbol: a.symbol,
                price_provider: a.price_provider,
                external_id: a.external_id,
            })
            .collect())
    }

    fn create_investment_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        amount: BigDecimal,
        strategy_id: Uuid,
        currency_id: Uuid,
    ) -> Result<InvestmentProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<InvestmentProposalDetails, diesel::result::Error, _>(|tx| {
            let proposal = diesel::insert_into(schema::proposal::table)
                .values(&NewProposalModel {
                    group_id,
                    created_by,
                })
                .returning(ProposalModel::as_returning())
                .get_result::<ProposalModel>(tx)?;

            diesel::insert_into(schema::investment_proposal::table)
                .values(&NewInvestmentProposalModel {
                    proposal_id: proposal.id,
                    amount: amount.clone(),
                    strategy_id,
                    currency_id,
                })
                .execute(tx)?;

            let proposal = diesel::update(
                schema::proposal::table.filter(schema::proposal::id.eq(proposal.id)),
            )
            .set(ProposalStatusUpdateModel {
                status: ProposalStatusModel::Approved,
            })
            .get_result::<ProposalModel>(tx)?;

            let strategy_name = schema::investment_strategy::table
                .filter(schema::investment_strategy::id.eq(strategy_id))
                .select(schema::investment_strategy::name)
                .first::<String>(tx)?;

            Ok(InvestmentProposalDetails {
                proposal_id: proposal.id,
                group_id: proposal.group_id,
                created_by: proposal.created_by,
                status: InvestmentStatus::Active,
                created_at: proposal.created_at,
                updated_at: proposal.updated_at,
                amount,
                strategy_id,
                currency_id,
                strategy_name,
            })
        })
        .map_err(|_| RepoError::Insert)
    }

    fn find_investment_proposal(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<InvestmentProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row =
            schema::investment_proposal::table
                .inner_join(
                    schema::proposal::table
                        .on(schema::investment_proposal::proposal_id.eq(schema::proposal::id)),
                )
                .inner_join(schema::investment_strategy::table.on(
                    schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id),
                ))
                .filter(schema::investment_proposal::proposal_id.eq(proposal_id))
                .first::<(
                    InvestmentProposalModel,
                    ProposalModel,
                    InvestmentStrategyModel,
                )>(&mut conn)
                .optional()
                .map_err(|_| RepoError::Query)?;
        Ok(row.map(|(ip, p, s)| InvestmentProposalDetails {
            proposal_id: p.id,
            group_id: p.group_id,
            created_by: p.created_by,
            status: InvestmentStatus::Active,
            created_at: p.created_at,
            updated_at: p.updated_at,
            amount: ip.amount,
            strategy_id: ip.strategy_id,
            currency_id: ip.currency_id,
            strategy_name: s.name,
        }))
    }

    fn list_approved_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<InvestmentProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows =
            schema::investment_proposal::table
                .inner_join(
                    schema::proposal::table
                        .on(schema::investment_proposal::proposal_id.eq(schema::proposal::id)),
                )
                .inner_join(schema::investment_strategy::table.on(
                    schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id),
                ))
                .filter(schema::proposal::group_id.eq(group_id))
                .filter(schema::proposal::status.eq(ProposalStatusModel::Approved))
                .select((
                    InvestmentProposalModel::as_select(),
                    ProposalModel::as_select(),
                    InvestmentStrategyModel::as_select(),
                ))
                .load::<(
                    InvestmentProposalModel,
                    ProposalModel,
                    InvestmentStrategyModel,
                )>(&mut conn)
                .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(ip, p, s)| InvestmentProposalDetails {
                proposal_id: p.id,
                group_id: p.group_id,
                created_by: p.created_by,
                status: InvestmentStatus::Active,
                created_at: p.created_at,
                updated_at: p.updated_at,
                amount: ip.amount,
                strategy_id: ip.strategy_id,
                currency_id: ip.currency_id,
                strategy_name: s.name,
            })
            .collect())
    }

    fn execute_investment(
        &self,
        proposal_id: Uuid,
        group_id: Uuid,
        _user_id: Uuid,
        amount: BigDecimal,
        strategy_id: Uuid,
        currency_id: Uuid,
        matures_at: NaiveDateTime,
        participants: Vec<NewInvestmentMember>,
        holdings: Vec<NewHolding>,
    ) -> Result<InvestmentDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<InvestmentDetails, diesel::result::Error, _>(|tx| {
            let debited = diesel::update(
                schema::group_wallet::table
                    .filter(schema::group_wallet::group_id.eq(group_id))
                    .filter(schema::group_wallet::currency_id.eq(currency_id))
                    .filter(schema::group_wallet::balance.ge(&amount)),
            )
            .set((
                schema::group_wallet::balance.eq(schema::group_wallet::balance - &amount),
                schema::group_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx)?;
            if debited != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            let wallet = schema::group_wallet::table
                .filter(schema::group_wallet::group_id.eq(group_id))
                .filter(schema::group_wallet::currency_id.eq(currency_id))
                .select((schema::group_wallet::id, schema::group_wallet::address))
                .first::<(Uuid, String)>(tx)?;

            diesel::update(schema::proposal::table.filter(schema::proposal::id.eq(proposal_id)))
                .set(ProposalStatusUpdateModel {
                    status: ProposalStatusModel::Executed,
                })
                .execute(tx)?;

            let now = chrono::Utc::now().naive_utc();
            let investment = diesel::insert_into(schema::investment::table)
                .values(&NewInvestmentModel {
                    id: Uuid::new_v4(),
                    proposal_id,
                    amount: amount.clone(),
                    current_value: amount.clone(),
                    status: InvestmentStatusModel::Active,
                    started_at: now,
                    matures_at,
                })
                .returning(InvestmentModel::as_returning())
                .get_result::<InvestmentModel>(tx)?;

            let strategy = schema::investment_strategy::table
                .filter(schema::investment_strategy::id.eq(strategy_id))
                .select(InvestmentStrategyModel::as_select())
                .first::<InvestmentStrategyModel>(tx)?;

            let member_models: Vec<NewInvestmentMemberModel> = participants
                .iter()
                .map(|p| NewInvestmentMemberModel {
                    investment_id: investment.id,
                    user_id: p.user_id.0,
                    balance_at_investment: p.balance_at_investment.clone(),
                    participation_pct: p.participation_pct.clone(),
                    invested_amount: p.invested_amount.clone(),
                    returned_amount: p.returned_amount.clone(),
                    withdrawn_at: p.withdrawn_at,
                })
                .collect();

            diesel::insert_into(schema::investment_member::table)
                .values(&member_models)
                .execute(tx)?;

            if !holdings.is_empty() {
                let holding_models: Vec<NewInvestmentHoldingModel> = holdings
                    .iter()
                    .map(|h| NewInvestmentHoldingModel {
                        investment_id: investment.id,
                        asset_id: h.asset_id,
                        units: h.units.clone(),
                        weight_bps_at_entry: h.weight_bps_at_entry,
                        cost_basis_usd: h.cost_basis_usd.clone(),
                    })
                    .collect();
                diesel::insert_into(schema::investment_holding::table)
                    .values(&holding_models)
                    .execute(tx)?;
            }

            for p in &participants {
                diesel::insert_into(schema::transaction::table)
                    .values(&NewTransactionModel {
                        tx_hash: None,
                        amount: p.invested_amount.clone(),
                        user_id: p.user_id.0,
                        group_id,
                        currency_id,
                        address: wallet.1.clone(),
                        description: Some("Investment execution".into()),
                        tx_type: TransactionTypeModel::from(TransactionType::Investment),
                    })
                    .execute(tx)?;
            }

            let holding_dtos = Self::load_holdings(tx, investment.id)?;

            Ok(Self::to_investment_details(
                investment,
                group_id,
                strategy_id,
                currency_id,
                &strategy,
                holding_dtos,
            ))
        })
        .map_err(|_| RepoError::Insert)
    }

    fn find_investment(&self, investment_id: Uuid) -> Result<Option<InvestmentDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row =
            schema::investment::table
                .filter(schema::investment::id.eq(investment_id))
                .inner_join(schema::investment_proposal::table.on(
                    schema::investment::proposal_id.eq(schema::investment_proposal::proposal_id),
                ))
                .inner_join(schema::investment_strategy::table.on(
                    schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id),
                ))
                .inner_join(
                    schema::proposal::table
                        .on(schema::investment::proposal_id.eq(schema::proposal::id)),
                )
                .select((
                    InvestmentModel::as_select(),
                    schema::investment_proposal::currency_id,
                    schema::investment_proposal::strategy_id,
                    schema::proposal::group_id,
                    InvestmentStrategyModel::as_select(),
                ))
                .first::<(InvestmentModel, Uuid, Uuid, Uuid, InvestmentStrategyModel)>(&mut conn)
                .optional()
                .map_err(|_| RepoError::Query)?;
        match row {
            Some((inv, currency_id, sid, gid, strategy)) => {
                let holdings =
                    Self::load_holdings(&mut conn, inv.id).map_err(|_| RepoError::Query)?;
                Ok(Some(Self::to_investment_details(
                    inv,
                    gid,
                    sid,
                    currency_id,
                    &strategy,
                    holdings,
                )))
            }
            None => Ok(None),
        }
    }

    fn list_group_investments(&self, group_id: Uuid) -> Result<Vec<InvestmentDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows =
            schema::investment::table
                .inner_join(
                    schema::proposal::table
                        .on(schema::investment::proposal_id.eq(schema::proposal::id)),
                )
                .filter(schema::proposal::group_id.eq(group_id))
                .inner_join(schema::investment_proposal::table.on(
                    schema::investment::proposal_id.eq(schema::investment_proposal::proposal_id),
                ))
                .inner_join(schema::investment_strategy::table.on(
                    schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id),
                ))
                .select((
                    InvestmentModel::as_select(),
                    schema::investment_proposal::currency_id,
                    schema::investment_proposal::strategy_id,
                    schema::proposal::group_id,
                    InvestmentStrategyModel::as_select(),
                ))
                .load::<(InvestmentModel, Uuid, Uuid, Uuid, InvestmentStrategyModel)>(&mut conn)
                .map_err(|_| RepoError::Query)?;
        let mut out = Vec::with_capacity(rows.len());
        for (inv, currency_id, sid, gid, strategy) in rows {
            let holdings = Self::load_holdings(&mut conn, inv.id).map_err(|_| RepoError::Query)?;
            out.push(Self::to_investment_details(
                inv,
                gid,
                sid,
                currency_id,
                &strategy,
                holdings,
            ));
        }
        Ok(out)
    }

    fn withdraw_investment(
        &self,
        investment_id: Uuid,
        group_id: Uuid,
        _user_id: Uuid,
        payout: BigDecimal,
        actual_return: BigDecimal,
        exit_kind: &str,
        fee_amount: BigDecimal,
    ) -> Result<InvestmentDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<InvestmentDetails, diesel::result::Error, _>(|tx| {
            let inv = schema::investment::table
                .filter(schema::investment::id.eq(investment_id))
                .select(InvestmentModel::as_select())
                .first::<InvestmentModel>(tx)?;

            let proposal = schema::proposal::table
                .filter(schema::proposal::id.eq(inv.proposal_id))
                .filter(schema::proposal::group_id.eq(group_id))
                .select(ProposalModel::as_select())
                .first::<ProposalModel>(tx)?;

            let ip = schema::investment_proposal::table
                .filter(schema::investment_proposal::proposal_id.eq(inv.proposal_id))
                .select(InvestmentProposalModel::as_select())
                .first::<InvestmentProposalModel>(tx)?;

            let hundred = BigDecimal::from(100);

            diesel::update(
                schema::group_wallet::table
                    .filter(schema::group_wallet::group_id.eq(proposal.group_id))
                    .filter(schema::group_wallet::currency_id.eq(ip.currency_id)),
            )
            .set((
                schema::group_wallet::balance.eq(schema::group_wallet::balance + &payout),
                schema::group_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx)?;

            let group_wallet_address = schema::group_wallet::table
                .filter(schema::group_wallet::group_id.eq(proposal.group_id))
                .filter(schema::group_wallet::currency_id.eq(ip.currency_id))
                .select(schema::group_wallet::address)
                .first::<String>(tx)?;

            let members = schema::investment_member::table
                .filter(schema::investment_member::investment_id.eq(investment_id))
                .select(InvestmentMemberModel::as_select())
                .load::<InvestmentMemberModel>(tx)?;

            let now = chrono::Utc::now().naive_utc();
            let desc = if exit_kind == "ragequit" {
                "Investment ragequit return"
            } else {
                "Investment return"
            };

            for member in &members {
                let member_return = payout.clone() * &member.participation_pct / &hundred;

                diesel::insert_into(schema::transaction::table)
                    .values(&NewTransactionModel {
                        tx_hash: None,
                        amount: member_return.clone(),
                        user_id: member.user_id,
                        group_id: proposal.group_id,
                        currency_id: ip.currency_id,
                        address: group_wallet_address.clone(),
                        description: Some(desc.into()),
                        tx_type: TransactionTypeModel::from(TransactionType::Deposit),
                    })
                    .execute(tx)?;

                diesel::update(
                    schema::investment_member::table
                        .filter(schema::investment_member::id.eq(member.id)),
                )
                .set((
                    schema::investment_member::returned_amount.eq(&member_return),
                    schema::investment_member::withdrawn_at.eq(now),
                ))
                .execute(tx)?;
            }

            let investment = diesel::update(
                schema::investment::table.filter(schema::investment::id.eq(investment_id)),
            )
            .set((
                schema::investment::status.eq(InvestmentStatusModel::Withdrawn),
                schema::investment::actual_return.eq(&actual_return),
                schema::investment::current_value.eq(&payout + &fee_amount),
                schema::investment::exit_kind.eq(exit_kind),
                schema::investment::fee_amount.eq(&fee_amount),
                schema::investment::updated_at.eq(now),
            ))
            .returning(InvestmentModel::as_returning())
            .get_result::<InvestmentModel>(tx)?;

            let strategy = schema::investment_strategy::table
                .filter(schema::investment_strategy::id.eq(ip.strategy_id))
                .select(InvestmentStrategyModel::as_select())
                .first::<InvestmentStrategyModel>(tx)?;

            let holdings = Self::load_holdings(tx, investment.id)?;

            Ok(Self::to_investment_details(
                investment,
                proposal.group_id,
                ip.strategy_id,
                ip.currency_id,
                &strategy,
                holdings,
            ))
        })
        .map_err(|_| RepoError::Insert)
    }

    fn list_snapshots(&self, investment_id: Uuid) -> Result<Vec<SnapshotDto>, RepoError> {
        use crate::infrastructure::db::models::investment::InvestmentValueSnapshotModel;

        let mut conn = self.get_conn()?;
        let rows = schema::investment_value_snapshot::table
            .filter(schema::investment_value_snapshot::investment_id.eq(investment_id))
            .order_by(schema::investment_value_snapshot::snapshot_date.asc())
            .select(InvestmentValueSnapshotModel::as_select())
            .load::<InvestmentValueSnapshotModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|s| SnapshotDto {
                investment_id: s.investment_id,
                value: s.value,
                snapshot_date: s.snapshot_date,
                created_at: s.created_at,
            })
            .collect())
    }

    fn list_active_with_strategy(&self) -> Result<Vec<ActiveInvestmentDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows =
            schema::investment::table
                .filter(schema::investment::status.eq(InvestmentStatusModel::Active))
                .inner_join(schema::investment_proposal::table.on(
                    schema::investment::proposal_id.eq(schema::investment_proposal::proposal_id),
                ))
                .inner_join(
                    schema::proposal::table
                        .on(schema::investment_proposal::proposal_id.eq(schema::proposal::id)),
                )
                .inner_join(schema::investment_strategy::table.on(
                    schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id),
                ))
                .select((
                    schema::investment::id,
                    schema::proposal::group_id,
                    schema::investment::amount,
                    schema::investment_strategy::expected_return_percentage,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::duration_days,
                    schema::investment_strategy::valuation_mode,
                    schema::investment_strategy::id,
                ))
                .load::<(
                    Uuid,
                    Uuid,
                    BigDecimal,
                    BigDecimal,
                    String,
                    i32,
                    String,
                    Uuid,
                )>(&mut conn)
                .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(
                |(id, group_id, amount, pct, risk, days, mode, strategy_id)| ActiveInvestmentDto {
                    id,
                    group_id,
                    amount,
                    expected_return_percentage: pct,
                    risk_level: risk,
                    duration_days: days,
                    valuation_mode: mode,
                    strategy_id,
                },
            )
            .collect())
    }

    fn list_holdings_for_pricing(
        &self,
        investment_id: Uuid,
    ) -> Result<Vec<(AssetPriceDto, BigDecimal)>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::investment_holding::table
            .filter(schema::investment_holding::investment_id.eq(investment_id))
            .inner_join(
                schema::asset::table.on(schema::investment_holding::asset_id.eq(schema::asset::id)),
            )
            .select((AssetModel::as_select(), schema::investment_holding::units))
            .load::<(AssetModel, BigDecimal)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(a, units)| {
                (
                    AssetPriceDto {
                        id: a.id,
                        symbol: a.symbol,
                        price_provider: a.price_provider,
                        external_id: a.external_id,
                    },
                    units,
                )
            })
            .collect())
    }

    fn count_snapshots(&self, investment_id: Uuid) -> Result<i64, RepoError> {
        let mut conn = self.get_conn()?;
        schema::investment_value_snapshot::table
            .filter(schema::investment_value_snapshot::investment_id.eq(investment_id))
            .count()
            .get_result(&mut conn)
            .map_err(|_| RepoError::Query)
    }

    fn update_current_value(
        &self,
        investment_id: Uuid,
        value: BigDecimal,
        now: NaiveDateTime,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        diesel::update(schema::investment::table.filter(schema::investment::id.eq(investment_id)))
            .set((
                schema::investment::current_value.eq(&value),
                schema::investment::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(())
    }

    fn mature_investment(
        &self,
        investment_id: Uuid,
        final_value: BigDecimal,
        actual_return: BigDecimal,
        now: NaiveDateTime,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        diesel::update(schema::investment::table.filter(schema::investment::id.eq(investment_id)))
            .set((
                schema::investment::status.eq(InvestmentStatusModel::Matured),
                schema::investment::actual_return.eq(&actual_return),
                schema::investment::current_value.eq(&final_value),
                schema::investment::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(())
    }

    fn insert_snapshot(
        &self,
        investment_id: Uuid,
        value: BigDecimal,
        snapshot_date: NaiveDateTime,
    ) -> Result<(), RepoError> {
        use crate::infrastructure::db::models::investment::NewInvestmentValueSnapshotModel;

        let mut conn = self.get_conn()?;
        diesel::insert_into(schema::investment_value_snapshot::table)
            .values(&NewInvestmentValueSnapshotModel {
                investment_id,
                value,
                snapshot_date,
            })
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;
        Ok(())
    }
}
