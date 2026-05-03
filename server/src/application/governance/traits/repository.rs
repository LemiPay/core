use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::governance::dto::{
    FundRoundContributionDetails, FundRoundProposalDetails, NewMemberProposalDetails,
    ProposalDetails, ReceivedNewMemberProposalDetails, WithdrawProposalDetails,
};
use crate::domain::governance::ProposalStatus;

pub trait GovernanceRepository: Send + Sync {
    fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        new_member_id: Uuid,
    ) -> Result<NewMemberProposalDetails, RepoError>;
    fn find_new_member_proposal_by_destination_and_group(
        &self,
        new_member_id: Uuid,
        group_id: Uuid,
    ) -> Result<Option<NewMemberProposalDetails>, RepoError>;
    fn find_new_member_proposal_by_id(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<NewMemberProposalDetails>, RepoError>;
    fn find_group_new_member_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, RepoError>;
    fn find_my_new_member_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, RepoError>;
    fn find_received_new_member_proposals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ReceivedNewMemberProposalDetails>, RepoError>;
    fn respond_new_member_proposal(
        &self,
        proposal_id: Uuid,
        destination: Uuid,
        next_status: ProposalStatus,
    ) -> Result<NewMemberProposalDetails, RepoError>;

    fn create_withdraw_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<WithdrawProposalDetails, RepoError>;
    fn find_withdraw_proposal(
        &self,
        proposal_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<WithdrawProposalDetails>, RepoError>;
    fn list_withdraw_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<WithdrawProposalDetails>, RepoError>;
    fn execute_withdraw(
        &self,
        proposal_id: Uuid,
        user_id: Uuid,
        group_id: Uuid,
        address: String,
        currency_id: Uuid,
        amount: BigDecimal,
    ) -> Result<(), RepoError>;

    fn create_fund_round_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        target_amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<FundRoundProposalDetails, RepoError>;
    fn find_fund_round(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<FundRoundProposalDetails>, RepoError>;
    fn list_fund_rounds(&self, group_id: Uuid) -> Result<Vec<FundRoundProposalDetails>, RepoError>;
    fn get_total_contributed(&self, proposal_id: Uuid) -> Result<BigDecimal, RepoError>;
    fn contribute_fund_round(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
        amount: BigDecimal,
        sender_wallet_id: Uuid,
    ) -> Result<FundRoundContributionDetails, RepoError>;
    fn find_user_contribution(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<FundRoundContributionDetails>, RepoError>;
    fn count_fund_round_contributors(&self, fund_round_id: Uuid) -> Result<i64, RepoError>;

    fn find_proposal(&self, proposal_id: Uuid) -> Result<Option<ProposalDetails>, RepoError>;
    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        status: ProposalStatus,
    ) -> Result<ProposalDetails, RepoError>;
}
