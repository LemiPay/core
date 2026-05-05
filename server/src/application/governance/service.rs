use std::{str::FromStr, sync::Arc};

use bigdecimal::BigDecimal;

use crate::application::{
    common::repo_error::RepoError, governance::error::GovernanceError,
    governance::traits::repository::GovernanceRepository,
    group::traits::repository::GroupRepository,
    treasury::traits::user_wallet_repo::UserWalletRepository,
    users::traits::repository::UserRepository,
};
use crate::domain::governance::{AutoApproveVotingPolicy, ProposalStatus, VotingPolicy};

#[derive(Clone)]
pub struct GovernanceService {
    pub governance_repo: Arc<dyn GovernanceRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl GovernanceService {
    pub(crate) fn parse_amount(raw: &str) -> Result<BigDecimal, GovernanceError> {
        BigDecimal::from_str(raw).map_err(|_| GovernanceError::InvalidAmount)
    }

    /// Returns the status a freshly created proposal should land in.
    /// Pluggable so the future voting system can override the policy.
    pub(crate) fn initial_proposal_status() -> ProposalStatus {
        AutoApproveVotingPolicy.initial_status()
    }

    pub(crate) fn map_repo<T>(result: Result<T, RepoError>) -> Result<T, GovernanceError> {
        result.map_err(GovernanceError::from)
    }
}
