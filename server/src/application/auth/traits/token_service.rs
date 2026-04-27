use crate::application::auth::jwt_token::JwtToken;
use crate::domain::user::UserId;

pub trait TokenService: Send + Sync {
    fn generate(&self, user_id: UserId) -> Result<JwtToken, TokenError>;

    fn verify(&self, token: &str) -> Result<UserId, TokenError>;
}

#[derive(Debug)]
pub enum TokenError {
    Invalid,
    Expired,
    Internal,
}
