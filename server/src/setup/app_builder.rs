use axum::Router;
use std::sync::Arc;

// bootstrap
use super::router::create_router;

use crate::setup::{
    builders::{
        auth::build_auth_service,
        group::build_group_service,
        users::build_user_service
    },
    state::AppState
};

use crate::{
    // infrastructure
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::{
            config::DbConfig,
            pool::{create_pool, DbPool},
            repositories::{
                auth_repo_impl::DieselAuthRepository, group_repo_impl::DieselGroupRepository,
                user_repo_impl::DieselUserRepository,
            },
        },
    },
    setup::config::AppConfig,
};

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
    let auth_repo = Arc::new(DieselAuthRepository::new(pool.clone()));
    let user_repo = Arc::new(DieselUserRepository::new(pool.clone()));
    let group_repo = Arc::new(DieselGroupRepository::new(pool.clone()));

    let hash_service = Arc::new(Argon2Hasher::new().expect("argon2 fail"));
    let token_service = Arc::new(JwtService::new(db_config.jwt_secret));

    // -------------------------
    // 4. Application Services
    // -------------------------
    let auth_service = build_auth_service(
        auth_repo.clone(),
        user_repo.clone(),
        hash_service,
        token_service,
    );

    let user_service = build_user_service(user_repo.clone());

    let group_service = build_group_service(group_repo.clone());

    // -------------------------
    // 5. State
    // -------------------------
    let state = Arc::new(AppState {
        config: app_config,

        // Services:
        auth_service,
        user_service,
        group_service,
    });

    // -------------------------
    // 6. Router
    // -------------------------
    create_router(state)
}
