use bigdecimal::BigDecimal;

use crate::domain::treasury::UserWalletId;
use crate::domain::user::UserId;

pub struct TransferFundsInput {
    pub user_id: UserId,
    pub sender_wallet_id: UserWalletId,
    pub receiver_address: String,
    pub amount: BigDecimal,
}
