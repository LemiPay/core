use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::{BigDecimal, Zero};

use crate::errors::app_error::AppError;
use crate::handlers::group_wallet::{
    ContributeFundRoundRequest, CreateFundRoundRequest, FundRoundStatusResponse,
};
use crate::models::proposal::{MyProposalStatus, NewProposal, ProposalUpdate};
use crate::models::proposals::fund_round::FundProposalExpanded;
use crate::repositories::traits::fund_round_repo::FundRoundRepository;
use crate::repositories::traits::group_repo::GroupRepository;

#[derive(Clone)]
pub struct GroupWalletService {
    fund_round_repo: Arc<dyn FundRoundRepository>,
    group_repo: Arc<dyn GroupRepository>,
}

impl GroupWalletService {
    pub fn new(
        fund_round_repo: Arc<dyn FundRoundRepository>,
        group_repo: Arc<dyn GroupRepository>,
    ) -> Self {
        Self {
            fund_round_repo,
            group_repo,
        }
    }

    pub fn create_fund_round(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        payload: CreateFundRoundRequest,
    ) -> Result<FundProposalExpanded, AppError> {
        let target_amount = BigDecimal::from_str(&payload.target_amount)
            .map_err(|_| AppError::BadRequest("Invalid target_amount".into()))?;

        if target_amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "target_amount must be greater than zero".into(),
            ));
        }

        // Validate group has wallet of that currency
        if !self
            .group_repo
            .has_wallet_with_currency(group_id, payload.currency_id)?
        {
            return Err(AppError::BadRequest(
                "Group does not have a wallet with the specified currency".into(),
            ));
        }

        let new_proposal = NewProposal {
            group_id,
            created_by,
        };

        let result = self.fund_round_repo.create_fund_round_proposal(
            new_proposal,
            target_amount,
            payload.currency_id,
        )?;

        Ok(result)
    }

    pub fn contribute_fund_round(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
        payload: ContributeFundRoundRequest,
    ) -> Result<FundRoundStatusResponse, AppError> {
        let amount = BigDecimal::from_str(&payload.amount)
            .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "amount must be greater than zero".into(),
            ));
        }

        let fund_round = self.find_fund_round(fund_round_id)?;

        if fund_round.proposal.status != MyProposalStatus::Approved {
            return Err(AppError::BadRequest("Fund round is not active".into()));
        }

        self.validate_is_member(user_id, fund_round.proposal.group_id)?;

        let total_contributed = self
            .fund_round_repo
            .get_total_contributed(fund_round_id)
            .map_err(AppError::Db)?;

        let remaining = &fund_round.fund_round_proposal.target_amount - &total_contributed;
        if amount > remaining {
            return Err(AppError::BadRequest(format!(
                "Amount exceeds remaining target. Remaining: {}",
                remaining
            )));
        }

        self.fund_round_repo
            .create_contribution(
                fund_round_id,
                user_id,
                amount.clone(),
                payload.sender_wallet_id,
            )
            .map_err(AppError::Db)?;

        let new_total = &total_contributed + &amount;
        let target = &fund_round.fund_round_proposal.target_amount;
        let is_completed = new_total >= *target;

        if is_completed {
            self.fund_round_repo
                .update_proposal_status(
                    fund_round_id,
                    ProposalUpdate {
                        status: MyProposalStatus::Executed,
                    },
                )
                .map_err(AppError::Db)?;
        }

        let updated = self.find_fund_round(fund_round_id)?;

        Ok(FundRoundStatusResponse {
            target_amount: updated.fund_round_proposal.target_amount.to_string(),
            total_contributed: new_total.to_string(),
            is_completed,
            fund_round: updated,
        })
    }

    pub fn get_fund_round_status(
        &self,
        fund_round_id: Uuid,
    ) -> Result<FundRoundStatusResponse, AppError> {
        let fund_round = self.find_fund_round(fund_round_id)?;

        let total_contributed = self
            .fund_round_repo
            .get_total_contributed(fund_round_id)
            .map_err(AppError::Db)?;

        let is_completed = fund_round.proposal.status == MyProposalStatus::Executed;

        Ok(FundRoundStatusResponse {
            target_amount: fund_round.fund_round_proposal.target_amount.to_string(),
            total_contributed: total_contributed.to_string(),
            is_completed,
            fund_round,
        })
    }

    pub fn cancel_fund_round(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
    ) -> Result<FundProposalExpanded, AppError> {
        let fund_round = self.find_fund_round(fund_round_id)?;

        if fund_round.proposal.status != MyProposalStatus::Approved {
            return Err(AppError::BadRequest("Fund round is not active".into()));
        }

        self.validate_is_admin(user_id, fund_round.proposal.group_id)?;

        self.fund_round_repo
            .update_proposal_status(
                fund_round_id,
                ProposalUpdate {
                    status: MyProposalStatus::Canceled,
                },
            )
            .map_err(AppError::Db)?;

        self.find_fund_round(fund_round_id)
    }

    fn find_fund_round(&self, fund_round_id: Uuid) -> Result<FundProposalExpanded, AppError> {
        self.fund_round_repo
            .find_fund_round(fund_round_id)
            .map_err(AppError::Db)?
            .ok_or(AppError::NotFound)
    }

    fn validate_is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<(), AppError> {
        if !self.group_repo.is_member(user_id, group_id)? {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }

    fn validate_is_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<(), AppError> {
        if !self.group_repo.is_admin(user_id, group_id)? {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }
}
