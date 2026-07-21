use super::super::{VerifyChallengeUseCase, dto::VerificationInput};
use crate::application::auth::new_user::NewUser;
use crate::application::auth::stored_user::StoredUser;
use crate::application::auth::traits::challenge_cache::{ChallengeCacheTrait, Web3AuthCacheTrait};
use crate::application::auth::traits::repository::AuthRepository;
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::application::common::repo_error::RepoError;
use crate::application::notifications::repository::NotificationRepository;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::group::GroupId;
use crate::domain::notification::types::NotificationRecordId;
use crate::domain::notification::{
    GroupNotificationPreference, NotificationChannel, NotificationEvent, NotificationRecord,
    UserNotificationPreference,
};
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::{Email, User, UserId};
use crate::infrastructure::auth::jwt_service::JwtService;
use crate::infrastructure::auth::web_3_auth::ChallengeData;
use crate::infrastructure::db::models::user::UserModel;
use crate::interfaces::http::error::AppError;
use alloy::primitives::Address;
use async_trait::async_trait;
use moka::sync::Cache;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub const EMAIL: &str = "user@example.com";
pub const ADDRESS: &str = "0x000000000000000000000000000000000000dead";
pub const NONCE: &str = "nonce-123";
pub const ISSUED_AT: &str = "2024-01-01T00:00:00Z";
pub const SIGNATURE: &str = "sig";
pub const NAME: &str = "Test User";

pub fn usdc_currency() -> CurrencyId {
    CurrencyId(Uuid::parse_str("33de6c7c-62a2-4182-813a-9005183be70d").expect("valid currency id"))
}

#[derive(Clone)]
pub struct UserFixture {
    pub id: UserId,
    pub email: String,
    pub name: String,
}

impl UserFixture {
    fn to_model(&self) -> UserModel {
        UserModel {
            id: self.id.0,
            email: self.email.clone(),
            password: None,
            name: self.name.clone(),
        }
    }
}

#[derive(Default)]
pub struct InMemoryUserRepo {
    users_by_email: Mutex<HashMap<String, UserModel>>,
    users_by_id: Mutex<HashMap<Uuid, UserModel>>,
}

impl InMemoryUserRepo {
    pub fn insert(&self, user: UserModel) {
        let UserModel {
            id,
            email,
            password,
            name,
        } = user;
        let by_email = UserModel {
            id,
            email: email.clone(),
            password: password.clone(),
            name: name.clone(),
        };
        let by_id = UserModel {
            id,
            email,
            password,
            name,
        };
        self.users_by_email
            .lock()
            .expect("users_by_email mutex poisoned")
            .insert(by_email.email.clone(), by_email);
        self.users_by_id
            .lock()
            .expect("users_by_id mutex poisoned")
            .insert(by_id.id, by_id);
    }
}

impl UserRepository for InMemoryUserRepo {
    fn find_by_email(&self, email: &Email) -> Result<Option<UserModel>, RepoError> {
        Ok(self
            .users_by_email
            .lock()
            .expect("users_by_email mutex poisoned")
            .get(&email.0)
            .map(|user| UserModel {
                id: user.id,
                email: user.email.clone(),
                password: user.password.clone(),
                name: user.name.clone(),
            }))
    }

    fn find_by_id(&self, user_id: &UserId) -> Result<Option<UserModel>, RepoError> {
        Ok(self
            .users_by_id
            .lock()
            .expect("users_by_id mutex poisoned")
            .get(user_id.as_uuid())
            .map(|user| UserModel {
                id: user.id,
                email: user.email.clone(),
                password: user.password.clone(),
                name: user.name.clone(),
            }))
    }
}

#[derive(Default)]
pub struct InMemoryWalletRepo {
    wallets_by_id: Mutex<HashMap<UserWalletId, UserWallet>>,
    wallets_by_address_currency: Mutex<HashMap<(String, CurrencyId), UserWallet>>,
    owners_by_address: Mutex<HashMap<String, UserId>>,
}

impl InMemoryWalletRepo {
    pub fn wallet_for_user(&self, user_id: UserId) -> Option<UserWallet> {
        self.wallets_for_user(user_id).into_iter().next()
    }

    pub fn wallets_for_user(&self, user_id: UserId) -> Vec<UserWallet> {
        self.wallets_by_id
            .lock()
            .expect("wallets_by_id mutex poisoned")
            .values()
            .filter(|wallet| wallet.user_id == user_id)
            .cloned()
            .collect()
    }
}

impl UserWalletRepository for InMemoryWalletRepo {
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
    ) -> Result<crate::application::treasury::dto::UserWalletDetails, RepoError> {
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
        Ok(crate::application::treasury::dto::UserWalletDetails {
            id: updated.id.0,
            address: updated.address,
            user_id: updated.user_id.0,
            currency_id: updated.balance.currency.0,
            balance: updated.balance.amount,
            created_at: now,
            updated_at: now,
        })
    }

    fn get_details(
        &self,
        _id: UserWalletId,
    ) -> Result<Option<crate::application::treasury::dto::UserWalletDetails>, RepoError> {
        Ok(None)
    }

    fn list_with_ticker_by_user(
        &self,
        _user_id: UserId,
    ) -> Result<Vec<crate::application::treasury::dto::UserWalletWithTickerDetails>, RepoError>
    {
        Ok(Vec::new())
    }
}

pub struct FakeAuthRepo {
    stored_user_id: UserId,
    saved_users: Mutex<Vec<NewUser>>,
}

impl FakeAuthRepo {
    pub fn new(stored_user_id: UserId) -> Self {
        Self {
            stored_user_id,
            saved_users: Mutex::new(Vec::new()),
        }
    }

    pub fn saved_users(&self) -> Vec<NewUser> {
        self.saved_users
            .lock()
            .expect("saved_users mutex poisoned")
            .iter()
            .map(|user| NewUser {
                email: user.email.clone(),
                password: user.password.clone(),
                name: user.name.clone(),
            })
            .collect()
    }
}

impl AuthRepository for FakeAuthRepo {
    fn save(&self, user: &NewUser) -> Result<StoredUser, RepoError> {
        self.saved_users
            .lock()
            .expect("saved_users mutex poisoned")
            .push(NewUser {
                email: user.email.clone(),
                password: user.password.clone(),
                name: user.name.clone(),
            });

        let stored_user = StoredUser {
            user: User::new(self.stored_user_id.0, user.name.clone(), user.email.clone())
                .expect("valid user"),
            password_hash: None,
        };

        Ok(stored_user)
    }
}

pub struct FakeWeb3Auth {
    cache: Mutex<HashMap<String, ChallengeData>>,
    signature_valid: Mutex<bool>,
}

impl FakeWeb3Auth {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            signature_valid: Mutex::new(true),
        }
    }

    pub fn insert_challenge(&self, address: &str, nonce: &str, issued_at: &str) {
        self.cache.lock().expect("cache mutex poisoned").insert(
            address.to_string(),
            ChallengeData {
                nonce: nonce.to_string(),
                issued_at: issued_at.to_string(),
            },
        );
    }

    pub fn set_signature_valid(&self, value: bool) {
        *self
            .signature_valid
            .lock()
            .expect("signature_valid mutex poisoned") = value;
    }

    pub fn has_challenge(&self, address: &str) -> bool {
        self.cache
            .lock()
            .expect("cache mutex poisoned")
            .contains_key(address)
    }
}

#[async_trait]
impl Web3AuthTrait for FakeWeb3Auth {
    fn generate_nonce(&self) -> String {
        NONCE.to_string()
    }

    fn generate_issued_at(&self) -> String {
        ISSUED_AT.to_string()
    }

    fn generate_message(&self, address: &Address, nonce: &String, issued_at: &String) -> String {
        format!("message:{}:{}:{}", address, nonce, issued_at)
    }

    async fn validate_signature_rpc(
        &self,
        _address: String,
        _signature_hex: String,
        _nonce: String,
        _issued_at: String,
    ) -> bool {
        *self
            .signature_valid
            .lock()
            .expect("signature_valid mutex poisoned")
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
pub struct InMemoryNotificationRepo;

impl NotificationRepository for InMemoryNotificationRepo {
    fn get_events(&self) -> Result<Vec<NotificationEvent>, RepoError> {
        Ok(vec![])
    }

    fn get_channels(&self) -> Result<Vec<NotificationChannel>, RepoError> {
        Ok(vec![])
    }

    fn get_user_preferences(
        &self,
        _user_id: UserId,
    ) -> Result<Vec<UserNotificationPreference>, RepoError> {
        Ok(vec![])
    }

    fn upsert_user_preference(
        &self,
        _preference: UserNotificationPreference,
    ) -> Result<(), RepoError> {
        Ok(())
    }

    fn get_group_preferences(
        &self,
        _user_id: UserId,
        _group_id: GroupId,
    ) -> Result<Vec<GroupNotificationPreference>, RepoError> {
        Ok(vec![])
    }

    fn upsert_group_preference(
        &self,
        _preference: GroupNotificationPreference,
    ) -> Result<(), RepoError> {
        Ok(())
    }

    fn initialize_defaults_for_user(&self, _user_id: UserId) -> Result<(), RepoError> {
        Ok(())
    }

    fn initialize_defaults_for_user_in_group(
        &self,
        _user_id: UserId,
        _group_id: GroupId,
    ) -> Result<(), RepoError> {
        Ok(())
    }

    fn list_user_notifications(
        &self,
        _user_id: UserId,
        _read_filter: Option<bool>,
        _limit: Option<i64>,
    ) -> Result<Vec<NotificationRecord>, RepoError> {
        Ok(vec![])
    }

    fn mark_notification_read(
        &self,
        _user_id: UserId,
        _notification_id: NotificationRecordId,
    ) -> Result<bool, RepoError> {
        Ok(false)
    }

    fn mark_all_notifications_read(&self, _user_id: UserId) -> Result<u64, RepoError> {
        Ok(0)
    }
}

pub struct TestContext {
    pub use_case: VerifyChallengeUseCase,
    pub web3: Arc<FakeWeb3Auth>,
    pub user_repo: Arc<InMemoryUserRepo>,
    pub wallet_repo: Arc<InMemoryWalletRepo>,
    pub auth_repo: Arc<FakeAuthRepo>,
    pub jwt_service: Arc<JwtService>,
    pub new_user_id: UserId,
    #[allow(dead_code)]
    pub notification_repo: Arc<InMemoryNotificationRepo>,
}

impl TestContext {
    pub fn new() -> Self {
        let web3 = Arc::new(FakeWeb3Auth::new());
        let user_repo = Arc::new(InMemoryUserRepo::default());
        let wallet_repo = Arc::new(InMemoryWalletRepo::default());
        let new_user_id = UserId(Uuid::new_v4());
        let auth_repo = Arc::new(FakeAuthRepo::new(new_user_id));
        let jwt_service = Arc::new(JwtService::new("test-secret".to_string()));
        let notification_repo = Arc::new(InMemoryNotificationRepo::default());
        let use_case = VerifyChallengeUseCase::new(
            web3.clone(),
            user_repo.clone(),
            wallet_repo.clone(),
            jwt_service.clone(),
            auth_repo.clone(),
            notification_repo.clone(),
        );

        Self {
            use_case,
            web3,
            user_repo,
            wallet_repo,
            auth_repo,
            jwt_service,
            new_user_id,
            notification_repo,
        }
    }

    pub fn given_existing_user(&self) -> UserFixture {
        let fixture = UserFixture {
            id: UserId(Uuid::new_v4()),
            email: EMAIL.to_string(),
            name: "Test User".to_string(),
        };
        self.user_repo.insert(fixture.to_model());
        fixture
    }

    pub fn given_wallet_for_user(&self, user: &UserFixture, address: &str) {
        let wallet = UserWallet {
            id: UserWalletId(Uuid::new_v4()),
            address: address.to_string(),
            user_id: user.id,
            balance: Money::zero(usdc_currency()),
        };
        self.wallet_repo.save(&wallet).expect("save wallet");
    }

    pub fn given_valid_challenge(&self) {
        self.given_challenge_with_nonce(NONCE);
        self.web3.set_signature_valid(true);
    }

    pub fn given_challenge_with_nonce(&self, nonce: &str) {
        self.web3.insert_challenge(ADDRESS, nonce, ISSUED_AT);
    }

    pub async fn verify(&self) -> Result<super::super::dto::VerificationOutput, AppError> {
        self.verify_with(VerificationInput {
            email: Some(EMAIL.to_string()),
            name: Some(NAME.to_string()),
            allow_linking: false,
            address: ADDRESS.to_string(),
            nonce: NONCE.to_string(),
            signature: SIGNATURE.to_string(),
            issued_at: Some(ISSUED_AT.to_string()),
        })
        .await
    }

    pub async fn verify_with(
        &self,
        input: VerificationInput,
    ) -> Result<super::super::dto::VerificationOutput, AppError> {
        self.use_case.verify_challenge(input).await
    }
}
