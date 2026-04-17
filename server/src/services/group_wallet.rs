use diesel::result::Error;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::{BigDecimal, Zero};

use crate::data::error::DbError;
use crate::errors::app_error::AppError;
use crate::handlers::group_wallet::NewGroupWalletRequest;
use crate::handlers::group_wallet::{
    ContributeFundRoundRequest, CreateFundRoundRequest, FundRoundStatusResponse,
};
use crate::models::group::group_wallet::{GroupWallet, NewGroupWallet};
use crate::models::proposal::{MyProposalStatus, NewProposal, ProposalUpdate};
use crate::models::proposals::fund_round::FundProposalExpanded;
use crate::repositories::traits::currency_repo::CurrencyRepository;
use crate::repositories::traits::fund_round_repo::FundRoundRepository;
use crate::repositories::traits::group_repo::GroupRepository;
use crate::repositories::traits::group_wallet_repo::GroupWalletRepository;
use crate::repositories::traits::proposal_repo::ProposalRepository;
use crate::repositories::traits::user_wallet_repo::UserWalletRepository;

#[derive(Clone)]
pub struct GroupWalletService {
    fund_round_repo: Arc<dyn FundRoundRepository>,
    currency_repo: Arc<dyn CurrencyRepository>,
    group_repo: Arc<dyn GroupRepository>,
    group_wallet_repo: Arc<dyn GroupWalletRepository>,
    proposal_repo: Arc<dyn ProposalRepository>,
    user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl GroupWalletService {
    pub fn new(
        fund_round_repo: Arc<dyn FundRoundRepository>,
        currency_repo: Arc<dyn CurrencyRepository>,
        group_repo: Arc<dyn GroupRepository>,
        group_wallet_repo: Arc<dyn GroupWalletRepository>,
        proposal_repo: Arc<dyn ProposalRepository>,
        user_wallet_repo: Arc<dyn UserWalletRepository>,
    ) -> Self {
        Self {
            fund_round_repo,
            currency_repo,
            group_repo,
            group_wallet_repo,
            proposal_repo,
            user_wallet_repo,
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
            .group_wallet_repo
            .get_wallet_by_group_and_currency(group_id, payload.currency_id)?
            .is_some()
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
    // -----

    fn parse_positive_amount(&self, raw: String) -> Result<BigDecimal, AppError> {
        let amount = BigDecimal::from_str(&raw)
            .map_err(|_| AppError::BadRequest("Invalid amount".into()))?;

        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "amount must be greater than zero".into(),
            ));
        }
        Ok(amount)
    }

    fn validate_active_fund_round(
        &self,
        fund_round_id: Uuid,
    ) -> Result<FundProposalExpanded, AppError> {
        let found = self.find_fund_round(fund_round_id)?;

        if found.proposal.status != MyProposalStatus::Approved {
            return Err(AppError::BadRequest("Fund round is not active".into()));
        }

        Ok(found)
    }

    fn resolve_group_wallet_for_fund_round(
        &self,
        fund_round: &FundProposalExpanded,
    ) -> Result<GroupWallet, AppError> {
        match self.group_wallet_repo.get_wallet_by_group_and_currency(
            fund_round.proposal.group_id,
            fund_round.fund_round_proposal.currency_id,
        )? {
            Some(wallet) => Ok(wallet),
            None => Err(AppError::BadRequest(
                "Group does not have a wallet with the specified currency".into(),
            )),
        }
    }

    fn validate_sender_wallet_for_contribution(
        &self,
        user_id: Uuid,
        sender_wallet_id: Uuid,
        expected_currency_id: Uuid,
        amount: &BigDecimal,
    ) -> Result<(), AppError> {
        let sender_wallet = match self.user_wallet_repo.get_wallet_info(sender_wallet_id) {
            Ok(wallet) => wallet,
            Err(DbError::Diesel(Error::NotFound)) => {
                return Err(AppError::BadRequest("Sender wallet not found".into()));
            }
            Err(err) => return Err(AppError::Db(err)),
        };

        if sender_wallet.user_id != user_id {
            return Err(AppError::BadRequest("Sender wallet not found".into()));
        }

        if sender_wallet.currency_id != expected_currency_id {
            return Err(AppError::BadRequest(
                "Sender wallet currency does not match fund round currency".into(),
            ));
        }

        // Pre-check UX (el repo igual lo revalida transaccionalmente)
        if sender_wallet.balance < *amount {
            return Err(AppError::BadRequest("Insufficient funds".into()));
        }

        Ok(())
    }

    pub fn contribute_fund_round(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
        payload: ContributeFundRoundRequest,
    ) -> Result<FundRoundStatusResponse, AppError> {
        // Validations
        let amount = self.parse_positive_amount(payload.amount)?; // Validate amount is positive
        let fund_round = self.validate_active_fund_round(fund_round_id)?; // Find & check if fund round is active
        let group_wallet = self.resolve_group_wallet_for_fund_round(&fund_round)?; // Find group wallet for fund round

        self.validate_is_member(user_id, fund_round.proposal.group_id)?;
        self.validate_sender_wallet_for_contribution(
            user_id,
            payload.sender_wallet_id,
            fund_round.fund_round_proposal.currency_id,
            &amount,
        )?;

        let contribute_result = match self.fund_round_repo.create_contribution(
            fund_round_id,
            user_id,
            amount,
            payload.sender_wallet_id,
            group_wallet,
        ) {
            Ok(result) => result,
            Err(DbError::Diesel(Error::NotFound)) => {
                return Err(AppError::BadRequest(
                    "Fund round is not active, insufficient funds, or amount exceeds remaining target".into(),
                ));
            }
            Err(err) => return Err(AppError::Db(err)),
        };

        let updated = self.find_fund_round(fund_round_id)?;

        Ok(FundRoundStatusResponse {
            target_amount: updated.fund_round_proposal.target_amount.to_string(),
            total_contributed: contribute_result.total_contributed.to_string(),
            is_completed: contribute_result.is_completed,
            fund_round: updated,
        })
    }

    pub fn get_fund_round_status(
        &self,
        user_id: Uuid,
        fund_round_id: Uuid,
    ) -> Result<FundRoundStatusResponse, AppError> {
        let fund_round = self.find_fund_round(fund_round_id)?;
        self.validate_is_member(user_id, fund_round.proposal.group_id)?;

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

        self.proposal_repo
            .update_proposal_status(
                fund_round_id,
                ProposalUpdate {
                    status: MyProposalStatus::Canceled,
                },
            )
            .map_err(AppError::Db)?;

        self.find_fund_round(fund_round_id)
    }

    pub fn create_wallet(
        &self,
        request: NewGroupWalletRequest,
        group_id: Uuid,
    ) -> Result<GroupWallet, AppError> {
        let currency_id = match self
            .currency_repo
            .get_currency_id_by_ticker(request.currency_ticker)
        {
            Ok(currency_id) => currency_id,
            Err(DbError::Diesel(Error::NotFound)) => {
                return Err(AppError::BadRequest("That currency doesn't exist".into()));
            }
            Err(err) => return Err(AppError::Db(err)),
        };

        let existing = self
            .group_wallet_repo
            .get_wallet_by_group_and_currency(group_id, currency_id)
            .map_err(AppError::Db)?;

        if existing.is_some() {
            return Err(AppError::BadRequest(
                "The group already has a wallet for this currency".into(),
            ));
        }

        let address_taken = self
            .group_wallet_repo
            .get_wallet_by_address_and_currency(&request.address, currency_id)
            .map_err(AppError::Db)?;

        if address_taken.is_some() {
            return Err(AppError::BadRequest(
                "That address is already registered for this currency".into(),
            ));
        }

        let new_wallet = NewGroupWallet {
            address: request.address,
            group_id,
            currency_id,
        };

        self.group_wallet_repo
            .create(new_wallet)
            .map_err(AppError::Db)
    }

    pub fn get_wallets_by_group(&self, group_id: Uuid) -> Result<Vec<GroupWallet>, AppError> {
        self.group_wallet_repo
            .get_wallets_by_group(group_id)
            .map_err(AppError::Db)
    }

    // Helpers
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
