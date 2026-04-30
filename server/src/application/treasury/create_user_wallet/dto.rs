use crate::application::treasury::dto::UserWalletDetails;
use crate::domain::user::UserId;

pub struct CreateUserWalletInput {
    pub address: String,
    pub currency_ticker: String,
    pub user_id: UserId,
}

pub struct CreateUserWalletOutput {
    pub wallet: UserWalletDetails,
}
