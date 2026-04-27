use axum::Router;
use std::sync::Arc;

use crate::{
    application::auth::{login::LoginUseCase, me::GetMeUseCase, register::RegisterUseCase},

    // infrastructure
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::{
            config::DbConfig,
            pool::{DbPool, create_pool},
            repositories::{
                // repositories
                auth_repo_impl::DieselAuthRepository,
            },
        },
    },

    setup::config::AppConfig,
};

// bootstrap
use super::{router::create_router, state::AppState};

pub fn build_app() -> Router {
    // -------------------------
    // 1. Config
    // -------------------------
    let db_config: DbConfig = DbConfig::from_env();
    let app_config: AppConfig = AppConfig::new();

    // -------------------------
    // 2. DB Pool
    // -------------------------
    let pool: DbPool = create_pool(&db_config.database_url);

    // -------------------------
    // 3. Infrastructure
    // -------------------------
    let auth_repo: Arc<DieselAuthRepository> = Arc::new(DieselAuthRepository::new(pool.clone()));

    let hasher = Arc::new(Argon2Hasher::new().expect("argon2 fail"));

    let token_service = Arc::new(JwtService::new(db_config.jwt_secret));

    // -------------------------
    // 4. Use cases
    // -------------------------
    let get_me_use_case = Arc::new(GetMeUseCase {
        repo: auth_repo.clone(),
    });

    let register_use_case = Arc::new(RegisterUseCase {
        repo: auth_repo.clone(),
        hash_service: hasher.clone(),
    });

    let login_use_case = Arc::new(LoginUseCase {
        repo: auth_repo.clone(),
        hash_service: hasher.clone(),
        token_service: token_service.clone(),
    });

    // -------------------------
    // 5. State
    // -------------------------
    let state = AppState {
        config: app_config,
        get_me_use_case,
        register_use_case,
        login_use_case,
    };

    // -------------------------
    // 6. Router
    // -------------------------
    create_router(state)
}
