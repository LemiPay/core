use super::super::{ChallengeUseCase, dto::ChallengeInput};
use crate::application::auth::traits::challenge_cache::{ChallengeCacheTrait, Web3AuthCacheTrait};
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::{UserWalletDetails, UserWalletWithTickerDetails};
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::UserId;
use crate::infrastructure::auth::web_3_auth::ChallengeData;
use crate::interfaces::http::error::AppError;
use alloy::primitives::Address;
use async_trait::async_trait;
use moka::sync::Cache;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub const ADDRESS: &str = "0x000000000000000000000000000000000000dead";
pub const NONCE: &str = "nonce-123";
pub const ISSUED_AT: &str = "2024-01-01T00:00:00Z";

pub struct FakeWeb3Auth {
    cache: Mutex<HashMap<String, ChallengeData>>,
    nonce: Mutex<String>,
    issued_at: Mutex<String>,
}

impl FakeWeb3Auth {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            nonce: Mutex::new(NONCE.to_string()),
            issued_at: Mutex::new(ISSUED_AT.to_string()),
        }
    }

    pub fn challenge_for(&self, address: &str) -> Option<ChallengeData> {
        self.cache
            .lock()
            .expect("cache mutex poisoned")
            .get(address)
            .cloned()
    }

    pub fn expected_message(
        &self,
        address: &Address,
        nonce: &String,
        issued_at: &String,
    ) -> String {
        format!("message:{}:{}:{}", address, nonce, issued_at)
    }
}

#[async_trait]
impl Web3AuthTrait for FakeWeb3Auth {
    fn generate_nonce(&self) -> String {
        self.nonce.lock().expect("nonce mutex poisoned").clone()
    }

    fn generate_issued_at(&self) -> String {
        self.issued_at
            .lock()
            .expect("issued_at mutex poisoned")
            .clone()
    }

    fn generate_message(&self, address: &Address, nonce: &String, issued_at: &String) -> String {
        self.expected_message(address, nonce, issued_at)
    }

    async fn validate_signature_rpc(
        &self,
        _address: String,
        _signature_hex: String,
        _nonce: String,
        _issued_at: String,
    ) -> bool {
        true
    }
}

#[async_trait]
impl ChallengeCacheTrait for FakeWeb3Auth {
    fn cache_get(&self, address: &String) -> Option<ChallengeData> {
        self.cache
            .lock()
            .expect("cache mutex poisoned")
            .get(address)
            .cloned()
    }

    fn cache_insert(&self, address: String, data: ChallengeData) {
        self.cache
            .lock()
            .expect("cache mutex poisoned")
            .insert(address, data);
    }

    fn cache_remove(&self, address: &String) {
        self.cache
            .lock()
            .expect("cache mutex poisoned")
            .remove(address);
    }

    fn new_cache() -> Cache<String, ChallengeData>
    where
        Self: Sized,
    {
        Cache::builder().max_capacity(10).build()
    }
}

impl Web3AuthCacheTrait for FakeWeb3Auth {}

#[derive(Default)]
pub struct FakeWalletRepo {
    wallets_by_id: Mutex<HashMap<UserWalletId, UserWallet>>,
    wallets_by_address_currency: Mutex<HashMap<(String, CurrencyId), UserWallet>>,
    owners_by_address: Mutex<HashMap<String, UserId>>,
}

impl UserWalletRepository for FakeWalletRepo {
    fn save(&self, wallet: &UserWallet) -> Result<(), RepoError> {
        self.wallets_by_id
            .lock()
            .expect("wallets_by_id mutex poisoned")
            .insert(wallet.id, wallet.clone());
        self.wallets_by_address_currency
            .lock()
            .expect("wallets_by_address_currency mutex poisoned")
            .insert(
                (wallet.address.clone(), wallet.balance.currency),
                wallet.clone(),
            );
        self.owners_by_address
            .lock()
            .expect("owners_by_address mutex poisoned")
            .insert(wallet.address.clone(), wallet.user_id);
        Ok(())
    }

    fn find_by_id(&self, id: UserWalletId) -> Result<Option<UserWallet>, RepoError> {
        Ok(self
            .wallets_by_id
            .lock()
            .expect("wallets_by_id mutex poisoned")
            .get(&id)
            .cloned())
    }

    fn find_by_address_and_currency(
        &self,
        address: &str,
        currency: CurrencyId,
    ) -> Result<Option<UserWallet>, RepoError> {
        Ok(self
            .wallets_by_address_currency
            .lock()
            .expect("wallets_by_address_currency mutex poisoned")
            .get(&(address.to_string(), currency))
            .cloned())
    }

    fn find_owner_of_address(&self, address: &str) -> Result<Option<UserId>, RepoError> {
        Ok(self
            .owners_by_address
            .lock()
            .expect("owners_by_address mutex poisoned")
            .get(address)
            .cloned())
    }

    fn transfer(
        &self,
        _sender: UserWalletId,
        _receiver: UserWalletId,
        _amount: &Money,
    ) -> Result<(), RepoError> {
        Ok(())
    }

    fn withdraw_atomic(
        &self,
        wallet_id: UserWalletId,
        amount: &Money,
    ) -> Result<UserWalletDetails, RepoError> {
        let mut wallets = self
            .wallets_by_id
            .lock()
            .expect("wallets_by_id mutex poisoned");
        let wallet = wallets.get(&wallet_id).cloned().ok_or(RepoError::Query)?;
        let updated = wallet.withdraw(amount).map_err(|_| RepoError::Insert)?;
        wallets.insert(wallet_id, updated.clone());
        self.wallets_by_address_currency
            .lock()
            .expect("wallets_by_address_currency mutex poisoned")
            .insert(
                (updated.address.clone(), updated.balance.currency),
                updated.clone(),
            );
        let now = chrono::Utc::now().naive_utc();
        Ok(UserWalletDetails {
            id: updated.id.0,
            address: updated.address,
            user_id: updated.user_id.0,
            currency_id: updated.balance.currency.0,
            balance: updated.balance.amount,
            created_at: now,
            updated_at: now,
        })
    }

    fn get_details(&self, _id: UserWalletId) -> Result<Option<UserWalletDetails>, RepoError> {
        Ok(None)
    }

    fn list_with_ticker_by_user(
        &self,
        _user_id: UserId,
    ) -> Result<Vec<UserWalletWithTickerDetails>, RepoError> {
        Ok(Vec::new())
    }
}

pub struct TestContext {
    pub use_case: ChallengeUseCase,
    pub web3: Arc<FakeWeb3Auth>,
}

impl TestContext {
    pub fn new() -> Self {
        let web3 = Arc::new(FakeWeb3Auth::new());
        let wallet_repo = Arc::new(FakeWalletRepo::default());
        let use_case = ChallengeUseCase::new(web3.clone(), wallet_repo);

        Self { use_case, web3 }
    }

    pub fn generate(&self, address: &str) -> Result<super::super::dto::ChallengeOutput, AppError> {
        self.use_case.generate_challenge(ChallengeInput {
            address: address.to_string(),
        })
    }
}
