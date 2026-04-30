use crate::application::treasury::dto::GroupWalletDetails;
use crate::domain::group::GroupId;

pub struct CreateGroupWalletInput {
    pub group_id: GroupId,
    pub address: String,
    pub currency_ticker: String,
}

pub struct CreateGroupWalletOutput {
    pub wallet: GroupWalletDetails,
}
