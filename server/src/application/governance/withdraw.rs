use uuid::Uuid;

use crate::application::governance::error::GovernanceError;
use crate::domain::governance::{
    GovernancePolicy, Proposal, ProposalId, ProposalKind, WithdrawProposal,
};
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::domain::user::UserId;

use super::{dto::WithdrawProposalDetails, service::GovernanceService};

impl GovernanceService {
    pub fn create_withdraw_proposal(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        address: String,
        amount: String,
        currency_id: Uuid,
    ) -> Result<WithdrawProposalDetails, GovernanceError> {
        let amount = Self::parse_amount(&amount)?;
        GovernancePolicy::ensure_positive_amount(&amount)?;

        Self::map_repo(
            self.user_wallet_repo
                .find_by_address_and_currency(&address, CurrencyId(currency_id)),
        )?
        .ok_or(GovernanceError::SenderWalletNotFound)?;

        Self::map_repo(self.governance_repo.create_withdraw_proposal(
            user_id,
            group_id,
            amount,
            currency_id,
        ))
    }

    pub fn execute_withdraw_proposal(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        address: String,
        proposal_id: Uuid,
        currency_id: Uuid,
    ) -> Result<(), GovernanceError> {
        let stored = Self::map_repo(
            self.governance_repo
                .find_withdraw_proposal(proposal_id, currency_id),
        )?
        .ok_or(GovernanceError::NotFound)?;

        let domain = to_domain_withdraw(&stored);
        GovernancePolicy::ensure_withdraw_can_execute(&domain, UserId(user_id), GroupId(group_id))?;

        Self::map_repo(self.governance_repo.execute_withdraw(
            proposal_id,
            user_id,
            group_id,
            address,
            currency_id,
            stored.amount,
        ))
    }

    pub fn list_withdraw_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<WithdrawProposalDetails>, GovernanceError> {
        Self::map_repo(self.governance_repo.list_withdraw_proposals(group_id))
    }
}

fn to_domain_withdraw(stored: &WithdrawProposalDetails) -> WithdrawProposal {
    WithdrawProposal {
        proposal: Proposal::rehydrate(
            ProposalId(stored.proposal.id),
            GroupId(stored.proposal.group_id),
            UserId(stored.proposal.created_by),
            stored.proposal.status,
            ProposalKind::Withdraw,
            stored.proposal.created_at,
            stored.proposal.updated_at,
        ),
        amount: stored.amount.clone(),
        currency_id: CurrencyId(stored.currency_id),
    }
}
