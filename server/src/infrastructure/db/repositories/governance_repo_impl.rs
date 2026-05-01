use bigdecimal::BigDecimal;
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    dsl::sum, prelude::*,
};
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    governance::{
        dto::{
            FundRoundContributionDetails, FundRoundProposalDetails, NewMemberProposalDetails,
            ProposalDetails, ReceivedNewMemberProposalDetails, WithdrawProposalDetails,
        },
        traits::repository::GovernanceRepository,
    },
};
use crate::domain::governance::{ProposalKind, ProposalStatus};
use crate::domain::treasury::TransactionType;
use crate::infrastructure::db::{
    models::{
        governance::{
            FundRoundContributionModel, FundRoundProposalModel, NewFundRoundContributionModel,
            NewMemberProposalModel, NewProposalModel, ProposalModel, ProposalStatusModel,
            ProposalStatusUpdateModel, WithdrawProposalModel,
        },
        treasury::{NewTransactionModel, TransactionTypeModel},
    },
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselGovernanceRepository {
    db: DbPool,
}

impl DieselGovernanceRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }

    fn to_proposal_details(proposal: ProposalModel) -> ProposalDetails {
        ProposalDetails {
            id: proposal.id,
            group_id: proposal.group_id,
            created_by: proposal.created_by,
            status: proposal.status.into(),
            created_at: proposal.created_at,
            updated_at: proposal.updated_at,
        }
    }
}

impl GovernanceRepository for DieselGovernanceRepository {
    fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        new_member_id: Uuid,
    ) -> Result<NewMemberProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        let result = conn
            .transaction::<NewMemberProposalDetails, diesel::result::Error, _>(|tx| {
                let mut created = diesel::insert_into(schema::proposal::table)
                    .values(&NewProposalModel {
                        group_id,
                        created_by,
                    })
                    .returning(ProposalModel::as_returning())
                    .get_result::<ProposalModel>(tx)?;

                let nmp = diesel::insert_into(schema::new_member_proposal::table)
                    .values(&NewMemberProposalModel {
                        proposal_id: created.id,
                        new_member_id,
                    })
                    .returning(NewMemberProposalModel::as_returning())
                    .get_result::<NewMemberProposalModel>(tx)?;

                created = diesel::update(
                    schema::proposal::table.filter(schema::proposal::id.eq(created.id)),
                )
                .set(ProposalStatusUpdateModel {
                    status: ProposalStatusModel::Approved,
                })
                .get_result::<ProposalModel>(tx)?;

                Ok(NewMemberProposalDetails {
                    proposal: Self::to_proposal_details(created),
                    new_member_id: nmp.new_member_id,
                    kind: ProposalKind::NewMember,
                })
            })
            .map_err(|_| RepoError::Insert)?;
        Ok(result)
    }

    fn find_new_member_proposal_by_destination_and_group(
        &self,
        new_member_id: Uuid,
        group_id: Uuid,
    ) -> Result<Option<NewMemberProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let result = schema::new_member_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::new_member_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::new_member_proposal::new_member_id.eq(new_member_id))
            .filter(schema::proposal::group_id.eq(group_id))
            .first::<(NewMemberProposalModel, ProposalModel)>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(result.map(|(nmp, p)| NewMemberProposalDetails {
            proposal: Self::to_proposal_details(p),
            new_member_id: nmp.new_member_id,
            kind: ProposalKind::NewMember,
        }))
    }

    fn find_new_member_proposal_by_id(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<NewMemberProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let result = schema::new_member_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::new_member_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::new_member_proposal::proposal_id.eq(proposal_id))
            .first::<(NewMemberProposalModel, ProposalModel)>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(result.map(|(nmp, p)| NewMemberProposalDetails {
            proposal: Self::to_proposal_details(p),
            new_member_id: nmp.new_member_id,
            kind: ProposalKind::NewMember,
        }))
    }

    fn find_group_new_member_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::new_member_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::new_member_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::proposal::group_id.eq(group_id))
            .load::<(NewMemberProposalModel, ProposalModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(nmp, p)| NewMemberProposalDetails {
                proposal: Self::to_proposal_details(p),
                new_member_id: nmp.new_member_id,
                kind: ProposalKind::NewMember,
            })
            .collect())
    }

    fn find_my_new_member_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::new_member_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::new_member_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::proposal::created_by.eq(created_by))
            .load::<(NewMemberProposalModel, ProposalModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(nmp, p)| NewMemberProposalDetails {
                proposal: Self::to_proposal_details(p),
                new_member_id: nmp.new_member_id,
                kind: ProposalKind::NewMember,
            })
            .collect())
    }

    fn find_received_new_member_proposals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ReceivedNewMemberProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::new_member_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::new_member_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .inner_join(schema::user::table.on(schema::proposal::created_by.eq(schema::user::id)))
            .inner_join(schema::group::table.on(schema::proposal::group_id.eq(schema::group::id)))
            .filter(schema::new_member_proposal::new_member_id.eq(user_id))
            .filter(schema::proposal::status.eq(ProposalStatusModel::Approved))
            .select((
                NewMemberProposalModel::as_select(),
                ProposalModel::as_select(),
                schema::user::name,
                schema::group::name,
            ))
            .load::<(NewMemberProposalModel, ProposalModel, String, String)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(
                |(nmp, p, sender_name, group_name)| ReceivedNewMemberProposalDetails {
                    sender_name,
                    group_name,
                    proposal: Self::to_proposal_details(p),
                    new_member_id: nmp.new_member_id,
                    kind: ProposalKind::NewMember,
                },
            )
            .collect())
    }

    fn respond_new_member_proposal(
        &self,
        proposal_id: Uuid,
        destination: Uuid,
        next_status: ProposalStatus,
    ) -> Result<NewMemberProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        let next_status_model: ProposalStatusModel = next_status.into();
        conn.transaction::<NewMemberProposalDetails, diesel::result::Error, _>(|tx| {
            let nmp = schema::new_member_proposal::table
                .filter(schema::new_member_proposal::proposal_id.eq(proposal_id))
                .get_result::<NewMemberProposalModel>(tx)?;

            let proposal = diesel::update(schema::proposal::table.filter(schema::proposal::id.eq(proposal_id)))
                .set(ProposalStatusUpdateModel { status: next_status_model })
                .get_result::<ProposalModel>(tx)?;

            if next_status_model == ProposalStatusModel::Executed {
                diesel::insert_into(schema::user_in_group::table)
                    .values((
                        schema::user_in_group::user_id.eq(destination),
                        schema::user_in_group::group_id.eq(proposal.group_id),
                        schema::user_in_group::role.eq(crate::infrastructure::db::models::group::GroupRoleModel::Member),
                    ))
                    .on_conflict((schema::user_in_group::user_id, schema::user_in_group::group_id))
                    .do_update()
                    .set((
                        schema::user_in_group::status.eq(
                            crate::infrastructure::db::models::group::GroupMemberStatusModel::Active,
                        ),
                        schema::user_in_group::role.eq(crate::infrastructure::db::models::group::GroupRoleModel::Member),
                    ))
                    .execute(tx)?;
            }

            Ok(NewMemberProposalDetails {
                proposal: Self::to_proposal_details(proposal),
                new_member_id: nmp.new_member_id,
                kind: ProposalKind::NewMember,
            })
        }).map_err(|_| RepoError::Insert)
    }

    fn create_withdraw_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<WithdrawProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<WithdrawProposalDetails, diesel::result::Error, _>(|tx| {
            let mut proposal = diesel::insert_into(schema::proposal::table)
                .values(&NewProposalModel {
                    group_id,
                    created_by,
                })
                .returning(ProposalModel::as_returning())
                .get_result::<ProposalModel>(tx)?;
            let wp = diesel::insert_into(schema::withdraw_proposal::table)
                .values(&WithdrawProposalModel {
                    proposal_id: proposal.id,
                    amount,
                    currency_id,
                })
                .returning(WithdrawProposalModel::as_returning())
                .get_result::<WithdrawProposalModel>(tx)?;

            proposal = diesel::update(
                schema::proposal::table.filter(schema::proposal::id.eq(proposal.id)),
            )
            .set(ProposalStatusUpdateModel {
                status: ProposalStatusModel::Approved,
            })
            .get_result::<ProposalModel>(tx)?;

            Ok(WithdrawProposalDetails {
                proposal: Self::to_proposal_details(proposal),
                amount: wp.amount,
                currency_id: wp.currency_id,
                kind: ProposalKind::Withdraw,
            })
        })
        .map_err(|_| RepoError::Insert)
    }

    fn find_withdraw_proposal(
        &self,
        proposal_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<WithdrawProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::withdraw_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::withdraw_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::withdraw_proposal::proposal_id.eq(proposal_id))
            .filter(schema::withdraw_proposal::currency_id.eq(currency_id))
            .first::<(WithdrawProposalModel, ProposalModel)>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(|(wp, p)| WithdrawProposalDetails {
            proposal: Self::to_proposal_details(p),
            amount: wp.amount,
            currency_id: wp.currency_id,
            kind: ProposalKind::Withdraw,
        }))
    }

    fn list_withdraw_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<WithdrawProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::withdraw_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::withdraw_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::proposal::group_id.eq(group_id))
            .load::<(WithdrawProposalModel, ProposalModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(wp, p)| WithdrawProposalDetails {
                proposal: Self::to_proposal_details(p),
                amount: wp.amount,
                currency_id: wp.currency_id,
                kind: ProposalKind::Withdraw,
            })
            .collect())
    }

    fn execute_withdraw(
        &self,
        proposal_id: Uuid,
        user_id: Uuid,
        group_id: Uuid,
        address: String,
        currency_id: Uuid,
        amount: BigDecimal,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<(), diesel::result::Error, _>(|tx| {
            let debited_group = diesel::update(
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
            if debited_group != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            let credited_user = diesel::update(
                schema::user_wallet::table
                    .filter(schema::user_wallet::user_id.eq(user_id))
                    .filter(schema::user_wallet::address.eq(&address))
                    .filter(schema::user_wallet::currency_id.eq(currency_id)),
            )
            .set((
                schema::user_wallet::balance.eq(schema::user_wallet::balance + &amount),
                schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx)?;
            if credited_user != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            let tx_row = NewTransactionModel {
                tx_hash: None,
                amount,
                user_id,
                group_id,
                currency_id,
                address,
                description: None,
                tx_type: TransactionTypeModel::from(TransactionType::Withdraw),
            };
            diesel::insert_into(schema::transaction::table)
                .values(&tx_row)
                .execute(tx)?;
            diesel::update(schema::proposal::table.filter(schema::proposal::id.eq(proposal_id)))
                .set(ProposalStatusUpdateModel {
                    status: ProposalStatusModel::Executed,
                })
                .execute(tx)?;
            Ok(())
        })
        .map_err(|_| RepoError::Insert)
    }

    fn create_fund_round_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        target_amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<FundRoundProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<FundRoundProposalDetails, diesel::result::Error, _>(|tx| {
            let mut created = diesel::insert_into(schema::proposal::table)
                .values(&NewProposalModel {
                    group_id,
                    created_by,
                })
                .returning(ProposalModel::as_returning())
                .get_result::<ProposalModel>(tx)?;
            let fr = diesel::insert_into(schema::fund_round_proposal::table)
                .values(&FundRoundProposalModel {
                    proposal_id: created.id,
                    target_amount,
                    currency_id,
                })
                .returning(FundRoundProposalModel::as_returning())
                .get_result::<FundRoundProposalModel>(tx)?;
            created =
                diesel::update(schema::proposal::table.filter(schema::proposal::id.eq(created.id)))
                    .set(ProposalStatusUpdateModel {
                        status: ProposalStatusModel::Approved,
                    })
                    .get_result::<ProposalModel>(tx)?;
            Ok(FundRoundProposalDetails {
                proposal: Self::to_proposal_details(created),
                target_amount: fr.target_amount,
                currency_id: fr.currency_id,
                kind: ProposalKind::FundRound,
            })
        })
        .map_err(|_| RepoError::Insert)
    }

    fn find_fund_round(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<FundRoundProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::fund_round_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::fund_round_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::fund_round_proposal::proposal_id.eq(proposal_id))
            .first::<(FundRoundProposalModel, ProposalModel)>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(|(fr, p)| FundRoundProposalDetails {
            proposal: Self::to_proposal_details(p),
            target_amount: fr.target_amount,
            currency_id: fr.currency_id,
            kind: ProposalKind::FundRound,
        }))
    }

    fn list_fund_rounds(&self, group_id: Uuid) -> Result<Vec<FundRoundProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::fund_round_proposal::table
            .inner_join(
                schema::proposal::table
                    .on(schema::fund_round_proposal::proposal_id.eq(schema::proposal::id)),
            )
            .filter(schema::proposal::group_id.eq(group_id))
            .load::<(FundRoundProposalModel, ProposalModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(rows
            .into_iter()
            .map(|(fr, p)| FundRoundProposalDetails {
                proposal: Self::to_proposal_details(p),
                target_amount: fr.target_amount,
                currency_id: fr.currency_id,
                kind: ProposalKind::FundRound,
            })
            .collect())
    }

    fn get_total_contributed(&self, proposal_id: Uuid) -> Result<BigDecimal, RepoError> {
        let mut conn = self.get_conn()?;
        let total: Option<BigDecimal> = schema::fund_round_contribution::table
            .filter(schema::fund_round_contribution::fund_round_proposal_id.eq(proposal_id))
            .select(sum(schema::fund_round_contribution::amount))
            .first(&mut conn)
            .map_err(|_| RepoError::Query)?;
        Ok(total.unwrap_or_default())
    }

    fn contribute_fund_round(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
        amount: BigDecimal,
        sender_wallet_id: Uuid,
    ) -> Result<FundRoundContributionDetails, RepoError> {
        let mut conn = self.get_conn()?;
        conn.transaction::<FundRoundContributionDetails, diesel::result::Error, _>(|tx| {
            let proposal = schema::proposal::table
                .filter(schema::proposal::id.eq(fund_round_id))
                .filter(schema::proposal::status.eq(ProposalStatusModel::Approved))
                .for_update()
                .select(ProposalModel::as_select())
                .first::<ProposalModel>(tx)?;
            let round = schema::fund_round_proposal::table
                .filter(schema::fund_round_proposal::proposal_id.eq(fund_round_id))
                .select(FundRoundProposalModel::as_select())
                .first::<FundRoundProposalModel>(tx)?;

            let total: Option<BigDecimal> = schema::fund_round_contribution::table
                .filter(schema::fund_round_contribution::fund_round_proposal_id.eq(fund_round_id))
                .select(sum(schema::fund_round_contribution::amount))
                .first(tx)?;
            let total = total.unwrap_or_default();
            if total.clone() + amount.clone() > round.target_amount {
                return Err(diesel::result::Error::NotFound);
            }

            let debited = diesel::update(
                schema::user_wallet::table
                    .filter(schema::user_wallet::id.eq(sender_wallet_id))
                    .filter(schema::user_wallet::user_id.eq(user_id))
                    .filter(schema::user_wallet::currency_id.eq(round.currency_id))
                    .filter(schema::user_wallet::balance.ge(&amount)),
            )
            .set((
                schema::user_wallet::balance.eq(schema::user_wallet::balance - &amount),
                schema::user_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx)?;
            if debited != 1 {
                return Err(diesel::result::Error::NotFound);
            }

            let wallet = schema::group_wallet::table
                .filter(schema::group_wallet::group_id.eq(proposal.group_id))
                .filter(schema::group_wallet::currency_id.eq(round.currency_id))
                .select((schema::group_wallet::id, schema::group_wallet::address))
                .first::<(Uuid, String)>(tx)?;

            diesel::update(
                schema::group_wallet::table.filter(schema::group_wallet::id.eq(wallet.0)),
            )
            .set((
                schema::group_wallet::balance.eq(schema::group_wallet::balance + &amount),
                schema::group_wallet::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(tx)?;

            let tx_id = diesel::insert_into(schema::transaction::table)
                .values(&NewTransactionModel {
                    tx_hash: None,
                    amount: amount.clone(),
                    user_id,
                    group_id: proposal.group_id,
                    currency_id: round.currency_id,
                    address: wallet.1,
                    description: Some("Fund round contribution".into()),
                    tx_type: TransactionTypeModel::from(TransactionType::Deposit),
                })
                .returning(schema::transaction::id)
                .get_result::<Uuid>(tx)?;

            let contribution = diesel::insert_into(schema::fund_round_contribution::table)
                .values(&NewFundRoundContributionModel {
                    fund_round_proposal_id: fund_round_id,
                    user_id,
                    amount: amount.clone(),
                    transaction_id: tx_id,
                })
                .returning(FundRoundContributionModel::as_returning())
                .get_result::<FundRoundContributionModel>(tx)?;

            if total + amount >= round.target_amount {
                diesel::update(
                    schema::proposal::table.filter(schema::proposal::id.eq(fund_round_id)),
                )
                .set(ProposalStatusUpdateModel {
                    status: ProposalStatusModel::Executed,
                })
                .execute(tx)?;
            }

            Ok(FundRoundContributionDetails {
                fund_round_proposal_id: contribution.fund_round_proposal_id,
                user_id: contribution.user_id,
                amount: contribution.amount,
                transaction_id: contribution.transaction_id,
                created_at: contribution.created_at,
                updated_at: contribution.updated_at,
            })
        })
        .map_err(|_| RepoError::Insert)
    }

    fn find_user_contribution(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<FundRoundContributionDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::fund_round_contribution::table
            .filter(schema::fund_round_contribution::fund_round_proposal_id.eq(fund_round_id))
            .filter(schema::fund_round_contribution::user_id.eq(user_id))
            .select(FundRoundContributionModel::as_select())
            .first::<FundRoundContributionModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(|c| FundRoundContributionDetails {
            fund_round_proposal_id: c.fund_round_proposal_id,
            user_id: c.user_id,
            amount: c.amount,
            transaction_id: c.transaction_id,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }))
    }

    fn count_fund_round_contributors(&self, fund_round_id: Uuid) -> Result<i64, RepoError> {
        let mut conn = self.get_conn()?;
        schema::fund_round_contribution::table
            .filter(schema::fund_round_contribution::fund_round_proposal_id.eq(fund_round_id))
            .count()
            .get_result(&mut conn)
            .map_err(|_| RepoError::Query)
    }

    fn find_proposal(&self, proposal_id: Uuid) -> Result<Option<ProposalDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::proposal::table
            .filter(schema::proposal::id.eq(proposal_id))
            .select(ProposalModel::as_select())
            .first::<ProposalModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(Self::to_proposal_details))
    }

    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        status: ProposalStatus,
    ) -> Result<ProposalDetails, RepoError> {
        let mut conn = self.get_conn()?;
        let row =
            diesel::update(schema::proposal::table.filter(schema::proposal::id.eq(proposal_id)))
                .set(ProposalStatusUpdateModel {
                    status: status.into(),
                })
                .get_result::<ProposalModel>(&mut conn)
                .map_err(|_| RepoError::Insert)?;
        Ok(Self::to_proposal_details(row))
    }
}
