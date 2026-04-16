use bigdecimal::BigDecimal;
use diesel::dsl::sum;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
// Models
use crate::models::group::group_wallet::GroupWallet;
use crate::models::proposal::{
    MyProposalStatus, NewProposal, Proposal, ProposalType, ProposalUpdate,
};
use crate::models::proposals::fund_round::{FundProposal, FundProposalExpanded};
use crate::models::transaction::fund_round_contrib::{
    FundRoundContribution, NewFundRoundContribution,
};
use crate::models::transaction::{MyTransactionType, NewTransaction, Transaction};

use crate::repositories::traits::fund_round_repo::FundRoundRepository;

use crate::schema::{
    fund_round_contribution, fund_round_proposal, group_wallet, proposal, transaction, user_wallet,
};

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
        group_wallet: GroupWallet,
    ) -> Result<FundRoundContribution, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<FundRoundContribution, DbError, _>(|tx_conn| {
            // Get fund round proposal and group id
            let (_, group_id) = fund_round_proposal::table
                .inner_join(proposal::table.on(fund_round_proposal::proposal_id.eq(proposal::id)))
                .filter(fund_round_proposal::proposal_id.eq(fund_round_id))
                .select((FundProposal::as_select(), proposal::group_id))
                .first::<(FundProposal, Uuid)>(tx_conn)?;

            let credited_rows = diesel::update(
                group_wallet::table
                    .filter(group_wallet::id.eq(group_wallet.id))
                    .filter(group_wallet::group_id.eq(group_id))
                    .filter(group_wallet::currency_id.eq(group_wallet.currency_id)),
            )
            .set(group_wallet::balance.eq(group_wallet::balance + amount.clone()))
            .execute(tx_conn)?;

            // Update user wallet balance
            let debited_rows = diesel::update(
                user_wallet::table
                    .filter(user_wallet::id.eq(sender_wallet_id))
                    .filter(user_wallet::user_id.eq(user_id))
                    .filter(user_wallet::balance.ge(amount.clone())),
            )
            .set(user_wallet::balance.eq(user_wallet::balance - amount.clone()))
            .execute(tx_conn)?;

            if credited_rows != 1 || debited_rows != 1 {
                return Err(diesel::result::Error::NotFound.into());
            }

            // Create tx
            let new_tx = NewTransaction {
                tx_hash: None,
                amount: amount.clone(),
                user_id,
                group_id,
                currency_id: group_wallet.currency_id,
                address: group_wallet.address,
                description: Some("Fund round contribution".to_string()),
                tx_type: MyTransactionType::Deposit,
            };

            // Insert tx
            let tx = diesel::insert_into(transaction::table)
                .values(&new_tx)
                .returning(Transaction::as_returning())
                .get_result(tx_conn)?;

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
                .get_result(tx_conn)?;

            Ok(contribution)
        })?;

        Ok(result)
    }
}
