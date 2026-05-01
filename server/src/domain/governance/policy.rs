use bigdecimal::{BigDecimal, Zero};

use crate::domain::governance::{
    error::GovernanceError,
    proposal::{FundRoundProposal, NewMemberProposal, Proposal, WithdrawProposal},
    status::ProposalStatus,
};
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub struct GovernancePolicy;

impl GovernancePolicy {
    pub fn ensure_positive_amount(amount: &BigDecimal) -> Result<(), GovernanceError> {
        if *amount <= BigDecimal::zero() {
            Err(GovernanceError::InvalidAmount)
        } else {
            Ok(())
        }
    }

    pub fn ensure_proposal_open(proposal: &Proposal) -> Result<(), GovernanceError> {
        if proposal.status.is_open() {
            Ok(())
        } else {
            Err(GovernanceError::ProposalAlreadyResolved)
        }
    }

    pub fn ensure_proposal_in_group(
        proposal: &Proposal,
        group_id: GroupId,
    ) -> Result<(), GovernanceError> {
        if proposal.group_id == group_id {
            Ok(())
        } else {
            Err(GovernanceError::GroupMismatch)
        }
    }

    pub fn ensure_proposal_creator(
        proposal: &Proposal,
        user_id: UserId,
    ) -> Result<(), GovernanceError> {
        if proposal.created_by == user_id {
            Ok(())
        } else {
            Err(GovernanceError::NotProposalCreator)
        }
    }

    pub fn ensure_destination_matches(
        proposal: &NewMemberProposal,
        user_id: UserId,
    ) -> Result<(), GovernanceError> {
        if proposal.new_member_id == user_id {
            Ok(())
        } else {
            Err(GovernanceError::NotProposalDestination)
        }
    }

    pub fn next_status_for_response(approve: bool) -> ProposalStatus {
        if approve {
            ProposalStatus::Executed
        } else {
            ProposalStatus::Rejected
        }
    }

    pub fn ensure_withdraw_can_execute(
        proposal: &WithdrawProposal,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<(), GovernanceError> {
        Self::ensure_proposal_in_group(&proposal.proposal, group_id)?;
        Self::ensure_proposal_creator(&proposal.proposal, user_id)?;
        if proposal.proposal.status != ProposalStatus::Approved {
            return Err(GovernanceError::ProposalAlreadyResolved);
        }
        Ok(())
    }

    pub fn ensure_fund_round_active(round: &FundRoundProposal) -> Result<(), GovernanceError> {
        if round.proposal.status == ProposalStatus::Approved {
            Ok(())
        } else {
            Err(GovernanceError::FundRoundNotActive)
        }
    }

    pub fn ensure_contribution_within_target(
        round: &FundRoundProposal,
        already_contributed: &BigDecimal,
        new_contribution: &BigDecimal,
    ) -> Result<(), GovernanceError> {
        let projected = already_contributed + new_contribution;
        if projected > round.target_amount {
            Err(GovernanceError::ContributionExceedsTarget)
        } else {
            Ok(())
        }
    }

    pub fn fund_round_remaining(
        round: &FundRoundProposal,
        total_contributed: &BigDecimal,
    ) -> Result<BigDecimal, GovernanceError> {
        let remaining = &round.target_amount - total_contributed;
        if remaining < BigDecimal::zero() {
            Err(GovernanceError::ContributionExceedsTarget)
        } else {
            Ok(remaining)
        }
    }

    pub fn is_last_pending_contributor(
        total_members: i64,
        contributors: i64,
        has_contributed: bool,
    ) -> bool {
        !has_contributed && (total_members - contributors) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::FromPrimitive;
    use uuid::Uuid;

    use crate::domain::governance::types::ProposalKind;
    use crate::domain::governance::{proposal::Proposal, types::ProposalId};

    fn make_proposal(group: GroupId, creator: UserId, status: ProposalStatus) -> Proposal {
        let now = chrono::Utc::now().naive_utc();
        Proposal::rehydrate(
            ProposalId(Uuid::new_v4()),
            group,
            creator,
            status,
            ProposalKind::NewMember,
            now,
            now,
        )
    }

    #[test]
    fn rejects_zero_amount() {
        let zero = BigDecimal::from_u64(0).unwrap();
        assert!(matches!(
            GovernancePolicy::ensure_positive_amount(&zero),
            Err(GovernanceError::InvalidAmount)
        ));
    }

    #[test]
    fn ensures_open_status() {
        let group = GroupId(Uuid::new_v4());
        let creator = UserId(Uuid::new_v4());
        let proposal = make_proposal(group, creator, ProposalStatus::Executed);
        assert!(matches!(
            GovernancePolicy::ensure_proposal_open(&proposal),
            Err(GovernanceError::ProposalAlreadyResolved)
        ));
    }

    #[test]
    fn next_status_for_response_maps_correctly() {
        assert_eq!(
            GovernancePolicy::next_status_for_response(true),
            ProposalStatus::Executed
        );
        assert_eq!(
            GovernancePolicy::next_status_for_response(false),
            ProposalStatus::Rejected
        );
    }
}
