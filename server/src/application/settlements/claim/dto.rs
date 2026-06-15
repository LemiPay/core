use bigdecimal::BigDecimal;

use crate::application::treasury::dto::TransactionDetails;
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::domain::user::UserId;

pub struct ClaimInput {
    pub user_id: UserId,
    pub group_id: GroupId,
    pub amount: BigDecimal,
    pub address: String,
    pub currency_id: CurrencyId,
    pub description: Option<String>,
}

pub struct ClaimOutput {
    pub transaction: TransactionDetails,
}
