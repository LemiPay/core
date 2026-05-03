use crate::application::auth::jwt_token::JwtToken;
use crate::domain::user::UserId;

pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub struct LoginOutput {
    pub user_id: UserId,
    pub token: JwtToken,
}
