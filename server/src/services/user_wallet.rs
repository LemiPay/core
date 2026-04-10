use bigdecimal::BigDecimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::models::user_wallet::{NewUserWallet, UserWallet};

use crate::repositories::traits::user_wallet_repo;
use crate::repositories::traits::user_wallet_repo::UserWalletRepository;

#[derive(Clone)]
pub struct UserWalletService {
    user_wallet_repo: Arc<dyn UserWalletRepository>,
}
impl UserWalletService {
    pub fn new(user_wallet_repo: Arc<dyn UserWalletRepository>) -> Self {
        Self { user_wallet_repo }
    }

    pub fn create_wallet(&self, new_user_wallet: NewUserWallet) -> Result<UserWallet, AppError> {
        //esta validación deberia irse cuando permitamos mas wallets por usuario
        let check_if_has_wallet = self
            .user_wallet_repo
            .get_user_wallet_address(new_user_wallet.user_id)?;
        if (check_if_has_wallet.is_some()) {
            return Err(AppError::Forbidden);
        }
        self.user_wallet_repo
            .create(new_user_wallet)
            .map_err(AppError::Db)
    }

    pub fn get_user_wallet(&self, user_id: Uuid) -> Result<UserWallet, AppError> {
        self.user_wallet_repo
            .get_user_wallet_address(user_id)
            .map_err(AppError::Db)?
            .ok_or(AppError::NotFound)
    }

    pub fn add_money_to_wallet(
        &self,
        user_id: Uuid,
        amount: BigDecimal,
        address: String,
    ) -> Result<UserWallet, AppError> {
        let check_ownership = self
            .user_wallet_repo
            .verify_user_owns_wallet(user_id, &*address)?;
        if !check_ownership {
            return Err(AppError::Forbidden);
        }
        self.user_wallet_repo
            .add_money_to_wallet(&*address, amount.clone())
            .map_err(AppError::Db)
    }
    pub fn take_money_from_wallet(
        &self,
        user_id: Uuid,
        amount: BigDecimal,
        address: String,
    ) -> Result<UserWallet, AppError> {
        //chequeo que el que lo hace es el dueño de la wallet
        let check_ownership = self
            .user_wallet_repo
            .verify_user_owns_wallet(user_id, &*address)?;
        if !check_ownership {
            return Err(AppError::Forbidden);
        }
        //chequeo que tenga saldo suficiente
        let balance = self.user_wallet_repo.get_balance_by_address(&*address)?;
        if balance < amount {
            return Err(AppError::Forbidden);
        }

        self.user_wallet_repo
            .take_money_by_address(&*address, amount.clone())
            .map_err(AppError::Db)
    }
    pub fn transfer_money_to_address(
        &self,
        sender_id: Uuid,
        amount: BigDecimal,
        sender_address: String,
        receiver_address: String,
    ) -> Result<bool, AppError> {
        let check_ownership = self
            .user_wallet_repo
            .verify_user_owns_wallet(sender_id, &*sender_address)?;
        if !check_ownership {
            return Err(AppError::Forbidden);
        }
        let balance = self
            .user_wallet_repo
            .get_balance_by_address(&*sender_address)?;
        if balance < amount {
            return Err(AppError::Forbidden);
        }
        self.user_wallet_repo
            .make_transfer_between_addresses(&*sender_address, &*receiver_address, amount.clone())
            .map_err(AppError::Db)
    }
    /**
     *## Esta funcion aplica para cuando queres el address de otro
     * no deberias saber su balance pero si su address linkeada
     */
    pub fn get_another_user_wallet_address(&self, user_id: Uuid) -> Result<String, AppError> {
        self.user_wallet_repo
            .get_user_wallet_address(user_id)
            .map_err(AppError::Db)?
            .map(|wallet| wallet.address)
            .ok_or(AppError::NotFound)
    }
}
