use crate::application::auth::challenge::ChallengeUseCase;
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::application::auth::verify_challenge::VerifyChallengeUseCase;
use crate::infrastructure::auth::web_3_auth::Web3Auth;
use crate::infrastructure::db::repositories::user_wallet_repo_impl::DieselUserWalletRepository;
use crate::{
    application::auth::{AuthService, login::LoginUseCase, register::RegisterUseCase},
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::repositories::auth_repo_impl::DieselAuthRepository,
        db::repositories::user_repo_impl::DieselUserRepository,
    },
};
use moka::sync::Cache;
use std::sync::Arc;
use std::time::Duration;

fn build_nonce_cache() -> Cache<String, String> {
    let cache = Cache::builder()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(900))
        .build();
    cache
}

pub fn build_auth_service(
    auth_repo: Arc<DieselAuthRepository>,
    user_repo: Arc<DieselUserRepository>,
    hash_service: Arc<Argon2Hasher>,
    token_service: Arc<JwtService>,
    web3_service: Arc<Web3Auth>,
    user_wallet_repository: Arc<DieselUserWalletRepository>,
) -> AuthService {
    AuthService {
        login: LoginUseCase {
            user_repo: user_repo.clone(),
            hash_service: hash_service.clone(),
            token_service: token_service.clone(),
        },
        register: RegisterUseCase {
            auth_repo: auth_repo.clone(),
            user_repo: user_repo.clone(),
            hash_service,
        },
        challenge: ChallengeUseCase::new(web3_service.clone(), build_nonce_cache()),
        verify_challenge: VerifyChallengeUseCase::new(
            web3_service.clone(),
            build_nonce_cache(),
            user_repo.clone(),
            user_wallet_repository.clone(),
            auth_repo.clone(),
            token_service.clone(),
        ),
    }
}
