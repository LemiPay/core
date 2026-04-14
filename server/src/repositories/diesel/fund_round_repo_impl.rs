use bigdecimal::BigDecimal;
use diesel::dsl::sum;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::proposal::{
    MyProposalStatus, NewProposal, Proposal, ProposalType, ProposalUpdate,
};
use crate::models::proposals::fund_round::{FundProposal, FundProposalExpanded};
use crate::models::transaction::fund_round_contrib::{FundRoundContribution, NewFundRoundContribution};
use crate::models::transaction::{MyTransactionType, NewTransaction, Transaction};
use crate::repositories::traits::fund_round_repo::FundRoundRepository;
use crate::schema::{fund_round_contribution, fund_round_proposal, proposal, transaction, user_wallet};

pub struct DieselFundRoundRepository {
    db: Db,
}

impl DieselFundRoundRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl FundRoundRepository for DieselFundRoundRepository {
    fn create_fund_round_proposal(
        &self,
        new_proposal: NewProposal,
        target_amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<FundProposalExpanded, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<FundProposalExpanded, DbError, _>(|this_conn| {
            // Create proposal
            let mut created_proposal = diesel::insert_into(proposal::table)
                .values(&new_proposal)
                .returning(Proposal::as_returning())
                .get_result(this_conn)?;

            // Create fund proposal
            let fund_proposal = FundProposal {
                proposal_id: created_proposal.id,
                target_amount,
                currency_id,
            };

            // Insert fund proposal
            let created_fund_proposal = diesel::insert_into(fund_round_proposal::table)
                .values(&fund_proposal)
                .returning(FundProposal::as_returning())
                .get_result(this_conn)?;

            // Update proposal status - Auto approve until voting system is implemented
            created_proposal =
                diesel::update(proposal::table.filter(proposal::id.eq(created_proposal.id)))
                    .set(ProposalUpdate {
                        status: MyProposalStatus::Approved,
                    })
                    .get_result(this_conn)?;

            Ok(FundProposalExpanded {
                proposal: created_proposal,
                fund_round_proposal: created_fund_proposal,
                proposal_type: ProposalType::FundRound,
            })
        })?;

        Ok(result)
    }

    fn find_fund_round(
        &self,
        fund_round_id: Uuid,
    ) -> Result<Option<FundProposalExpanded>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = fund_round_proposal::table
            .inner_join(proposal::table.on(fund_round_proposal::proposal_id.eq(proposal::id)))
            .filter(fund_round_proposal::proposal_id.eq(fund_round_id))
            .first::<(FundProposal, Proposal)>(&mut conn)
            .optional()?;

        Ok(result.map(|(frp, p)| FundProposalExpanded {
            proposal: p,
            fund_round_proposal: frp,
            proposal_type: ProposalType::FundRound,
        }))
    }

    fn get_total_contributed(&self, fund_round_id: Uuid) -> Result<BigDecimal, DbError> {
        let mut conn = self.db.get_conn()?;

        let total: Option<BigDecimal> = fund_round_contribution::table
            .filter(fund_round_contribution::fund_round_proposal_id.eq(fund_round_id))
            .select(sum(fund_round_contribution::amount))
            .first(&mut conn)?;

        Ok(total.unwrap_or_default())
    }

    fn create_contribution(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
        amount: BigDecimal,
        sender_wallet_id: Uuid,
    ) -> Result<FundRoundContribution, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<FundRoundContribution, DbError, _>(|this_conn| {
            // Get fund round proposal and group id
            let (frp, group_id) = fund_round_proposal::table
                .inner_join(proposal::table.on(fund_round_proposal::proposal_id.eq(proposal::id)))
                .filter(fund_round_proposal::proposal_id.eq(fund_round_id))
                .select((FundProposal::as_select(), proposal::group_id))
                .first::<(FundProposal, Uuid)>(this_conn)?;

            let currency_id = frp.currency_id;

            // Update user wallet balance
            diesel::update(
                user_wallet::table
                    .filter(user_wallet::id.eq(sender_wallet_id))
                    .filter(user_wallet::user_id.eq(user_id))
                    .filter(user_wallet::balance.ge(amount.clone())),
            )
            .set(user_wallet::balance.eq(user_wallet::balance - amount.clone()))
            .get_result::<crate::models::user_wallet::UserWallet>(this_conn)?;

            // Create tx
            let new_tx = NewTransaction {
                tx_hash: None,
                amount: amount.clone(),
                user_id,
                group_id,
                currency_id,
                description: Some("Fund round contribution".to_string()),
                tx_type: MyTransactionType::Deposit,
            };

            // Insert tx
            let tx = diesel::insert_into(transaction::table)
                .values(&new_tx)
                .returning(Transaction::as_returning())
                .get_result(this_conn)?;

            let new_contribution = NewFundRoundContribution {
                fund_round_proposal_id: fund_round_id,
                user_id,
                amount,
                transaction_id: tx.id,
            };

            // Insert contribution
            let contribution = diesel::insert_into(fund_round_contribution::table)
                .values(&new_contribution)
                .returning(FundRoundContribution::as_returning())
                .get_result(this_conn)?;

            Ok(contribution)
        })?;

        Ok(result)
    }

    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        params: ProposalUpdate,
    ) -> Result<Proposal, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = diesel::update(proposal::table.filter(proposal::id.eq(proposal_id)))
            .set(params)
            .get_result::<Proposal>(&mut conn)?;

        Ok(result)
    }
}
