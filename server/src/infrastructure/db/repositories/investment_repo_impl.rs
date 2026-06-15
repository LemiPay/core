use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, prelude::*,
};
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    investment::dto::{
        ActiveInvestmentDto, InvestmentDetails, InvestmentProposalDetails, InvestmentStrategyDto,
        SnapshotDto,
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
            InvestmentMemberModel, InvestmentModel, InvestmentProposalModel, InvestmentStatusModel,
            InvestmentStrategyModel, NewInvestmentMemberModel, NewInvestmentModel,
            NewInvestmentProposalModel,
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

    fn to_investment_details(
        inv: InvestmentModel,
        group_id: Uuid,
        strategy_id: Uuid,
        currency_id: Uuid,
        strategy_name: String,
        risk_level: String,
        expected_return_percentage: BigDecimal,
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
            strategy_name,
            risk_level,
            expected_return_percentage,
        }
    }
}

impl InvestmentRepository for DieselInvestmentRepository {
    // ── Strategies ──

    fn list_strategies(&self) -> Result<Vec<InvestmentStrategyDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::investment_strategy::table
            .select(InvestmentStrategyModel::as_select())
            .load::<InvestmentStrategyModel>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|s| InvestmentStrategyDto {
                id: s.id,
                name: s.name,
                description: s.description,
                risk_level: s.risk_level,
                expected_return_percentage: s.expected_return_percentage,
                duration_days: s.duration_days,
                created_at: s.created_at,
            })
            .collect())
    }

    fn find_strategy(&self, strategy_id: Uuid) -> Result<Option<InvestmentStrategyDto>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::investment_strategy::table
            .filter(schema::investment_strategy::id.eq(strategy_id))
            .select(InvestmentStrategyModel::as_select())
            .first::<InvestmentStrategyModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(|s| InvestmentStrategyDto {
            id: s.id,
            name: s.name,
            description: s.description,
            risk_level: s.risk_level,
            expected_return_percentage: s.expected_return_percentage,
            duration_days: s.duration_days,
            created_at: s.created_at,
        }))
    }

    // ── Investment Proposals ──

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

    // ── Execute Investment ──

    fn execute_investment(
        &self,
        proposal_id: Uuid,
        group_id: Uuid,
        _user_id: Uuid,
        amount: BigDecimal,
        strategy_id: Uuid,
        currency_id: Uuid,
        matures_at: NaiveDateTime,
        partipants: Vec<NewInvestmentMember>,
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
                .select((
                    schema::investment_strategy::name,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::expected_return_percentage,
                ))
                .first::<(String, String, BigDecimal)>(tx)?;

            let member_models: Vec<NewInvestmentMemberModel> = partipants
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

            for p in &partipants {
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

            Ok(Self::to_investment_details(
                investment,
                group_id,
                strategy_id,
                currency_id,
                strategy.0,
                strategy.1,
                strategy.2,
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
                    schema::investment_strategy::name,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::expected_return_percentage,
                ))
                .first::<(
                    InvestmentModel,
                    Uuid,
                    Uuid,
                    Uuid,
                    String,
                    String,
                    BigDecimal,
                )>(&mut conn)
                .optional()
                .map_err(|_| RepoError::Query)?;
        Ok(row.map(|(inv, currency_id, sid, gid, name, risk, pct)| {
            Self::to_investment_details(inv, gid, sid, currency_id, name, risk, pct)
        }))
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
                    schema::investment_strategy::name,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::expected_return_percentage,
                ))
                .load::<(
                    InvestmentModel,
                    Uuid,
                    Uuid,
                    Uuid,
                    String,
                    String,
                    BigDecimal,
                )>(&mut conn)
                .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(inv, currency_id, sid, gid, name, risk, pct)| {
                Self::to_investment_details(inv, gid, sid, currency_id, name, risk, pct)
            })
            .collect())
    }

    fn withdraw_investment(
        &self,
        investment_id: Uuid,
        group_id: Uuid,
        _user_id: Uuid,
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

            let total_return = inv.amount.clone() + inv.actual_return.clone().unwrap_or_default();
            let hundred = BigDecimal::from(100);

            diesel::update(
                schema::group_wallet::table
                    .filter(schema::group_wallet::group_id.eq(proposal.group_id))
                    .filter(schema::group_wallet::currency_id.eq(ip.currency_id)),
            )
            .set((
                schema::group_wallet::balance.eq(schema::group_wallet::balance + &total_return),
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

            for member in &members {
                let member_return = total_return.clone() * &member.participation_pct / &hundred;

                diesel::insert_into(schema::transaction::table)
                    .values(&NewTransactionModel {
                        tx_hash: None,
                        amount: member_return.clone(),
                        user_id: member.user_id,
                        group_id: proposal.group_id,
                        currency_id: ip.currency_id,
                        address: group_wallet_address.clone(),
                        description: Some("Investment return".into()),
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
                schema::investment::updated_at.eq(now),
            ))
            .returning(InvestmentModel::as_returning())
            .get_result::<InvestmentModel>(tx)?;

            let strategy = schema::investment_strategy::table
                .filter(schema::investment_strategy::id.eq(ip.strategy_id))
                .select((
                    schema::investment_strategy::name,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::expected_return_percentage,
                ))
                .first::<(String, String, BigDecimal)>(tx)?;

            Ok(Self::to_investment_details(
                investment,
                proposal.group_id,
                ip.strategy_id,
                ip.currency_id,
                strategy.0,
                strategy.1,
                strategy.2,
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

    // ── Pulse ──

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
                ))
                .load::<(Uuid, Uuid, BigDecimal, BigDecimal, String, i32)>(&mut conn)
                .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(
                |(id, group_id, amount, pct, risk, days)| ActiveInvestmentDto {
                    id,
                    group_id,
                    amount,
                    expected_return_percentage: pct,
                    risk_level: risk,
                    duration_days: days,
                },
            )
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
