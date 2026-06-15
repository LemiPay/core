use crate::domain::user::Email;
use async_trait::async_trait;

#[derive(Debug)]
pub enum EmailServiceError {
    SendFailed,
    Internal,
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn example(&self, to: &Email) -> Result<(), EmailServiceError>;

    async fn send_welcome_email(&self, to: &Email, name: &str) -> Result<(), EmailServiceError>;

    async fn send_login_alert(&self, to: &Email, name: &str) -> Result<(), EmailServiceError>;

    // Business event notifications (specific methods per event type, as requested)
    async fn send_proposal_created_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
    async fn send_proposal_approved_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
    async fn send_proposal_rejected_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
    async fn send_proposal_executed_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;

    async fn send_new_member_added_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;

    async fn send_fund_round_created_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
    async fn send_investment_created_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
    async fn send_expense_created_email(
        &self,
        to: &Email,
        group_name: &str,
    ) -> Result<(), EmailServiceError>;
}
