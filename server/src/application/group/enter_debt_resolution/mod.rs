pub mod dto;
use std::sync::Arc;

use crate::application::balances::BalancesService;
use crate::application::governance::traits::repository::GovernanceRepository;
use crate::application::group::{
    enter_debt_resolution::dto::{EnterDebtResolutionInput, EnterDebtResolutionOutput},
    traits::repository::GroupRepository,
};
use crate::application::investment::traits::repository::InvestmentRepository;
use crate::domain::governance::ProposalStatus;
use crate::domain::group::{GroupError, GroupPolicy};
use crate::domain::investment::InvestmentStatus;

#[derive(Debug)]
pub enum EnterDebtResolutionError {
    NotFound,
    Forbidden,
    NotActive,
    ActiveInvestments,
    ActiveProposals(String),
    Internal,
}

impl From<GroupError> for EnterDebtResolutionError {
    fn from(err: GroupError) -> Self {
        match err {
            GroupError::NotMember | GroupError::NotAdmin => EnterDebtResolutionError::Forbidden,
            GroupError::GroupNotActive => EnterDebtResolutionError::NotActive,
            _ => EnterDebtResolutionError::Internal,
        }
    }
}

#[derive(Clone)]
pub struct EnterDebtResolutionUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub investment_repo: Arc<dyn InvestmentRepository>,
    pub governance_repo: Arc<dyn GovernanceRepository>,
    pub balances_service: BalancesService,
}

impl EnterDebtResolutionUseCase {
    pub fn execute(
        &self,
        input: EnterDebtResolutionInput,
    ) -> Result<EnterDebtResolutionOutput, EnterDebtResolutionError> {
        let group = self
            .group_repo
            .find_by_id(input.group_id)
            .map_err(|_| EnterDebtResolutionError::Internal)?
            .ok_or(EnterDebtResolutionError::NotFound)?;

        // Validate permissions/status before any side-effects.
        GroupPolicy::can_enter_debt_resolution(input.actor_id, &group)
            .map_err(EnterDebtResolutionError::from)?;
        let investments = self
            .investment_repo
            .list_group_investments(input.group_id.0)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        let has_non_withdrawn = investments
            .iter()
            .any(|inv| inv.status != InvestmentStatus::Withdrawn);

        if has_non_withdrawn {
            return Err(EnterDebtResolutionError::ActiveInvestments);
        }

        let mut issues: Vec<&str> = Vec::new();

        let withdraw_proposals = self
            .governance_repo
            .list_withdraw_proposals(input.group_id.0)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        if withdraw_proposals
            .iter()
            .any(|p| p.proposal.status.is_open())
        {
            issues.push("propuestas de retiro pendientes");
        }

        let investment_proposals = self
            .investment_repo
            .list_approved_proposals(input.group_id.0)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        if !investment_proposals.is_empty() {
            issues.push("propuestas de inversión sin ejecutar");
        }

        if !issues.is_empty() {
            let msg = format!(
                "Hay {}. Ejecutalas o cancelalas antes de cerrar el grupo.",
                issues.join(" y ")
            );
            return Err(EnterDebtResolutionError::ActiveProposals(msg));
        }

        let new_member_proposals = self
            .governance_repo
            .find_group_new_member_proposals(input.group_id.0)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        for p in &new_member_proposals {
            if p.proposal.status.is_open() {
                self.governance_repo
                    .update_proposal_status(p.proposal.id, ProposalStatus::Canceled)
                    .map_err(|_| EnterDebtResolutionError::Internal)?;
            }
        }

        let balances = self
            .balances_service
            .get_balances(input.group_id)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        let balances_map = balances.to_domain();

        let updated = if GroupPolicy::can_end_group(balances_map.clone()).is_ok() {
            group
                .deactivate(balances_map)
                .map_err(|_| EnterDebtResolutionError::Internal)?
        } else {
            group
                .enter_debt_resolution(input.actor_id)
                .map_err(EnterDebtResolutionError::from)?
        };

        self.group_repo
            .save(&updated)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        self.group_repo
            .get_group_details(input.group_id)
            .map_err(|_| EnterDebtResolutionError::Internal)?
            .ok_or(EnterDebtResolutionError::NotFound)
            .map(|group| EnterDebtResolutionOutput { group })
    }
}
