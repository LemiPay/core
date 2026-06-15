use std::sync::Arc;

use crate::{
    application::auth::{
        AuthService, challenge::ChallengeUseCase, login::LoginUseCase, register::RegisterUseCase,
        traits::challenge_cache::Web3AuthCacheTrait, verify_challenge::VerifyChallengeUseCase,
    },
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::repositories::{
            auth_repo_impl::DieselAuthRepository,
            notifications_repo_impl::DieselNotificationRepository,
            user_repo_impl::DieselUserRepository,
            user_wallet_repo_impl::DieselUserWalletRepository,
        },
    },
};

pub fn build_auth_service(
    auth_repo: Arc<DieselAuthRepository>,
    user_repo: Arc<DieselUserRepository>,
    hash_service: Arc<Argon2Hasher>,
    token_service: Arc<JwtService>,
    web3_service: Arc<dyn Web3AuthCacheTrait>,
    user_wallet_repository: Arc<DieselUserWalletRepository>,
    notification_repo: Arc<DieselNotificationRepository>,
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
            notification_repo: notification_repo.clone(),
        },
        challenge: ChallengeUseCase::new(web3_service.clone(), user_wallet_repository.clone()),
        verify_challenge: VerifyChallengeUseCase::new(
            web3_service.clone(),
            user_repo.clone(),
            user_wallet_repository.clone(),
            token_service.clone(),
            auth_repo.clone(),
            notification_repo.clone(),
        ),
    }
}
