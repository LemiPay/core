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
}
