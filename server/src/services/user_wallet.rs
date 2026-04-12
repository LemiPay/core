use crate::errors::app_error::AppError;
use crate::handlers::user_wallet::{AddressGroup, FundTransferRequest, NewWalletRequest};
use crate::models::user_wallet::{NewUserWallet, UserWallet, WalletWithTickerDb};
use crate::repositories::traits::currency_repo::CurrencyRepository;
use crate::repositories::traits::user_wallet_repo::UserWalletRepository;
use bigdecimal::{BigDecimal, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserWalletService {
    user_wallet_repo: Arc<dyn UserWalletRepository>,
    currency_repo: Arc<dyn CurrencyRepository>,
}
impl UserWalletService {
    pub fn new(
        user_wallet_repo: Arc<dyn UserWalletRepository>,
        currency_repo: Arc<dyn CurrencyRepository>,
    ) -> Self {
        Self {
            user_wallet_repo,
            currency_repo,
        }
    }

    pub fn create_wallet(
        &self,
        wallet_request: NewWalletRequest,
        user_id: Uuid,
    ) -> Result<UserWallet, AppError> {
        //chequeo si existe esa currency en la db
        let currency_id = self
            .currency_repo
            .check_if_currency_exist(wallet_request.currency_ticker.clone())
            .map_err(|_| AppError::BadRequest("that currency doesn't exist".into()))?;

        //chequeo si el user ya tiene esa address asignada por otra currency
        let owner_opt = self
            .user_wallet_repo
            .get_owner_of_address(&wallet_request.address)
            .map_err(AppError::Db)?;

        if let Some(owner_id) = owner_opt {
            if owner_id != user_id {
                return Err(AppError::Unauthorized);
            }
        }

        //chequeo que esa wallet no este tambien con esa currency (asi no salta el db error)
        let existing_currency_wallet = self
            .user_wallet_repo
            .get_wallet_id_by_address_and_currency(&wallet_request.address, currency_id)
            .map_err(AppError::Db)?;

        if existing_currency_wallet.is_some() {
            return Err(AppError::Forbidden);
        }

        //la creo
        let new_user_wallet = NewUserWallet {
            address: wallet_request.address,
            user_id,
            currency_id,
        };

        self.user_wallet_repo
            .create(new_user_wallet)
            .map_err(AppError::Db)
    }
    pub fn faucet_fund_wallet(
        &self,
        user_id: Uuid,
        wallet_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, AppError> {
        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "El monto a fondear debe ser mayor a cero".into(),
            ));
        }

        let wallet = self
            .user_wallet_repo
            .get_wallet_info(wallet_id)
            .map_err(AppError::Db)?;

        if wallet.user_id != user_id {
            return Err(AppError::Forbidden);
        }
        self.user_wallet_repo
            .add_money_to_wallet(wallet_id, amount)
            .map_err(AppError::Db)
    }
    pub fn faucet_withdraw_wallet(
        &self,
        current_user_id: Uuid,
        wallet_id: Uuid,
        amount: BigDecimal,
    ) -> Result<UserWallet, AppError> {
        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "El monto a fondear debe ser mayor a cero".into(),
            ));
        }
        let wallet = self
            .user_wallet_repo
            .get_wallet_info(wallet_id)
            .map_err(AppError::Db)?;

        if wallet.user_id != current_user_id {
            return Err(AppError::Unauthorized);
        }
        if wallet.balance < amount {
            return Err(AppError::BadRequest(
                "Saldo insuficiente para realizar el retiro".into(),
            ));
        }

        self.user_wallet_repo
            .take_money_by_address(wallet_id, amount)
            .map_err(AppError::Db)
    }
    pub fn transfer_funds(
        &self,
        current_user_id: Uuid,
        request: FundTransferRequest,
    ) -> Result<bool, AppError> {
        let amount = BigDecimal::from_str(&request.amount)
            .map_err(|_| AppError::BadRequest("Monto inválido".into()))?;

        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest(
                "El monto a transferir debe ser mayor a cero".into(),
            ));
        }

        let sender_wallet = self
            .user_wallet_repo
            .get_wallet_info(request.sender_wallet_id)
            .map_err(AppError::Db)?;

        if sender_wallet.user_id != current_user_id {
            return Err(AppError::Unauthorized);
        }

        if sender_wallet.balance < amount {
            return Err(AppError::BadRequest("Saldo insuficiente".into()));
        }

        if sender_wallet.address == request.receiver_address {
            return Err(AppError::BadRequest(
                "No podés transferir a la misma dirección".into(),
            ));
        }

        let receiver_wallet = self
            .user_wallet_repo
            .get_wallet_id_by_address_and_currency(
                &request.receiver_address,
                sender_wallet.currency_id,
            )
            .map_err(AppError::Db)?;

        let receiver_wallet_id = match receiver_wallet {
            Some(id) => id,
            None => {
                return Err(AppError::BadRequest(
                    "La dirección de destino no existe o no soporta esta moneda".into(),
                ));
            }
        };

        self.user_wallet_repo
            .make_transfer_between_wallets(request.sender_wallet_id, receiver_wallet_id, amount)
            .map_err(AppError::Db)
    }
    pub fn get_my_wallet_info(
        &self,
        user_id: Uuid,
        wallet_id: Uuid,
    ) -> Result<UserWallet, AppError> {
        let wallet = self
            .user_wallet_repo
            .get_wallet_info(wallet_id)
            .map_err(AppError::Db)?;

        if wallet.user_id != user_id {
            return Err(AppError::Unauthorized);
        }
        Ok(wallet)
    }
    pub fn get_my_wallet_by_address_and_ticker(
        &self,
        current_user_id: Uuid,
        address: &str,
        ticker: String,
    ) -> Result<UserWallet, AppError> {
        let currency_id = self
            .currency_repo
            .check_if_currency_exist(ticker)
            .map_err(|_| AppError::BadRequest("that currency doesn't exist".into()))?;

        let wallet_id = self
            .user_wallet_repo
            .get_wallet_id_by_address_and_currency(address, currency_id)
            .map_err(AppError::Db)?
            .ok_or(AppError::NotFound)?;

        let wallet = self
            .user_wallet_repo
            .get_wallet_info(wallet_id)
            .map_err(AppError::Db)?;

        if wallet.user_id != current_user_id {
            return Err(AppError::Unauthorized);
        }

        Ok(wallet)
    }
    pub fn get_wallet_id_by_address_and_ticker(
        &self,
        user_id: Uuid,
        address: &str,
        ticker: String,
    ) -> Result<Uuid, AppError> {
        let currency_id = self.currency_repo.check_if_currency_exist(ticker)?;

        let wallet_id_opt = self
            .user_wallet_repo
            .get_wallet_id_by_address_and_currency(address, currency_id)
            .map_err(AppError::Db)?;

        let wallet_id = wallet_id_opt.ok_or(AppError::NotFound)?;

        let is_owner = self
            .user_wallet_repo
            .verify_user_owns_wallet(user_id, address)
            .map_err(AppError::Db)?;

        if !is_owner {
            return Err(AppError::Unauthorized);
        }

        Ok(wallet_id)
    }

    //esta forma de agruparlo lo hice para que sea mas facil de ver en el front dsp
    //sino pq como me lo imagino, habria que elgir la addres y sobre eso las currencies a usar
    //mas facil tenerlo ya mapeado
    pub fn get_grouped_user_wallets(
        &self,
        current_user_id: Uuid,
    ) -> Result<Vec<AddressGroup>, AppError> {
        let all_wallets_db = self
            .user_wallet_repo
            .get_all_wallets_by_user(current_user_id)
            .map_err(AppError::Db)?;

        let mut grouped_map: HashMap<String, Vec<WalletWithTickerDb>> = HashMap::new();

        for wallet_row in all_wallets_db {
            let detail = WalletWithTickerDb {
                address: wallet_row.address.to_string(),
                wallet_id: wallet_row.wallet_id,
                ticker: wallet_row.ticker,
                balance: wallet_row.balance,
            };

            grouped_map
                .entry(wallet_row.address)
                .or_insert_with(Vec::new)
                .push(detail);
        }

        let final_response = grouped_map
            .into_iter()
            .map(|(address, currencies)| AddressGroup {
                address,
                currencies,
            })
            .collect();

        Ok(final_response)
    }
}
