use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::application::governance::error::GovernanceError;
use crate::domain::governance::{
    FundRoundProposal, GovernancePolicy, Proposal, ProposalId, ProposalKind, ProposalStatus,
};
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::domain::user::UserId;

use super::{
    dto::{FundRoundContributionDetails, FundRoundProposalDetails},
    service::GovernanceService,
};

impl GovernanceService {
    pub fn create_fund_round_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        target_amount: String,
        currency_id: Uuid,
    ) -> Result<FundRoundProposalDetails, GovernanceError> {
        let target_amount = Self::parse_amount(&target_amount)?;
        GovernancePolicy::ensure_positive_amount(&target_amount)?;

        Self::map_repo(self.governance_repo.create_fund_round_proposal(
            created_by,
            group_id,
            target_amount,
            currency_id,
        ))
    }

    pub fn contribute_fund_round(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
        amount: String,
        sender_wallet_id: Uuid,
    ) -> Result<FundRoundContributionDetails, GovernanceError> {
        let amount = Self::parse_amount(&amount)?;
        GovernancePolicy::ensure_positive_amount(&amount)?;

        let stored = Self::map_repo(self.governance_repo.find_fund_round(fund_round_id))?
            .ok_or(GovernanceError::NotFound)?;
        let round = to_domain_fund_round(&stored);
        GovernancePolicy::ensure_fund_round_active(&round)?;

        let already_contributed =
            Self::map_repo(self.governance_repo.get_total_contributed(fund_round_id))?;
        GovernancePolicy::ensure_contribution_within_target(&round, &already_contributed, &amount)?;

        Self::map_repo(self.governance_repo.contribute_fund_round(
            fund_round_id,
            user_id,
            amount,
            sender_wallet_id,
        ))
    }

    pub fn find_fund_round_status(
        &self,
        fund_round_id: Uuid,
    ) -> Result<(FundRoundProposalDetails, BigDecimal, bool), GovernanceError> {
        let stored = Self::map_repo(self.governance_repo.find_fund_round(fund_round_id))?
            .ok_or(GovernanceError::NotFound)?;
        let total = Self::map_repo(self.governance_repo.get_total_contributed(fund_round_id))?;
        let is_completed = stored.proposal.status == ProposalStatus::Executed;
        Ok((stored, total, is_completed))
    }

    pub fn find_fund_round_remaining(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
    ) -> Result<(BigDecimal, bool), GovernanceError> {
        let (stored, total, _) = self.find_fund_round_status(fund_round_id)?;
        let round = to_domain_fund_round(&stored);
        let remaining = GovernancePolicy::fund_round_remaining(&round, &total)?;

        let total_members = Self::map_repo(
            self.group_repo
                .get_group_members(GroupId(stored.proposal.group_id)),
        )?
        .len() as i64;
        let contributors = Self::map_repo(
            self.governance_repo
                .count_fund_round_contributors(fund_round_id),
        )?;
        let has_contributed = Self::map_repo(
            self.governance_repo
                .find_user_contribution(fund_round_id, user_id),
        )?
        .is_some();
        let is_last_contributor = GovernancePolicy::is_last_pending_contributor(
            total_members,
            contributors,
            has_contributed,
        );
        Ok((remaining, is_last_contributor))
    }

    pub fn get_user_fund_round_contribution(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
    ) -> Result<FundRoundContributionDetails, GovernanceError> {
        Self::map_repo(
            self.governance_repo
                .find_user_contribution(fund_round_id, user_id),
        )?
        .ok_or(GovernanceError::NotFound)
    }

    pub fn list_fund_rounds(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<FundRoundProposalDetails>, GovernanceError> {
        Self::map_repo(self.governance_repo.list_fund_rounds(group_id))
    }

    pub fn cancel_fund_round(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
    ) -> Result<FundRoundProposalDetails, GovernanceError> {
        let stored = Self::map_repo(self.governance_repo.find_fund_round(fund_round_id))?
            .ok_or(GovernanceError::NotFound)?;
        let round = to_domain_fund_round(&stored);

        GovernancePolicy::ensure_proposal_creator(&round.proposal, UserId(user_id))?;
        GovernancePolicy::ensure_fund_round_active(&round)?;
        round
            .proposal
            .status
            .ensure_can_transition_to(ProposalStatus::Canceled)?;

        Self::map_repo(
            self.governance_repo
                .update_proposal_status(fund_round_id, ProposalStatus::Canceled),
        )?;
        Self::map_repo(self.governance_repo.find_fund_round(fund_round_id))?
            .ok_or(GovernanceError::NotFound)
    }
}

fn to_domain_fund_round(stored: &FundRoundProposalDetails) -> FundRoundProposal {
    FundRoundProposal {
        proposal: Proposal::rehydrate(
            ProposalId(stored.proposal.id),
            GroupId(stored.proposal.group_id),
            UserId(stored.proposal.created_by),
            stored.proposal.status,
            ProposalKind::FundRound,
            stored.proposal.created_at,
            stored.proposal.updated_at,
        ),
        target_amount: stored.target_amount.clone(),
        currency_id: CurrencyId(stored.currency_id),
    }
}
