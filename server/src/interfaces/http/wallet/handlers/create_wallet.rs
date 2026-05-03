use axum::{Json, extract::State};

use crate::application::treasury::create_user_wallet::dto::CreateUserWalletInput;
use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    wallet::dto::{CreateWalletRequest, UserWalletResponse},
};
use crate::setup::state::SharedState;

pub async fn create_wallet(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<CreateWalletRequest>,
) -> Result<Json<UserWalletResponse>, AppError> {
    let output = state
        .treasury_service
        .create_user_wallet
        .execute(CreateUserWalletInput {
            address: req.address,
            currency_ticker: req.currency_ticker,
            user_id: user.user_id,
        })
        .map_err(AppError::from)?;

    let w = output.wallet;
    Ok(Json(UserWalletResponse {
        id: w.id,
        address: w.address,
        user_id: w.user_id,
        currency_id: w.currency_id,
        balance: w.balance,
        created_at: w.created_at,
        updated_at: w.updated_at,
    }))
}
