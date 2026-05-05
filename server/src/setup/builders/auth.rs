use std::sync::Arc;

use crate::application::auth::challenge::ChallengeUseCase;
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::infrastructure::auth::web_3_auth::Web3Auth;
use crate::{
    application::auth::{AuthService, login::LoginUseCase, register::RegisterUseCase},
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::repositories::auth_repo_impl::DieselAuthRepository,
        db::repositories::user_repo_impl::DieselUserRepository,
    },
};

pub fn build_auth_service(
    auth_repo: Arc<DieselAuthRepository>,
    user_repo: Arc<DieselUserRepository>,
    hash_service: Arc<Argon2Hasher>,
    token_service: Arc<JwtService>,
    web3_service: Arc<Web3Auth>,
) -> AuthService {
    AuthService {
        login: LoginUseCase {
            user_repo: user_repo.clone(),
            hash_service: hash_service.clone(),
            token_service,
        },
        register: RegisterUseCase {
            auth_repo,
            user_repo,
            hash_service,
        },
        challenge: ChallengeUseCase::new(web3_service.clone()),
    }
}
