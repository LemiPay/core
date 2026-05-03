use axum::Router;
use std::sync::Arc;

// bootstrap
use super::router::create_router;

use crate::setup::{
    builders::{
        auth::build_auth_service, balances::build_balances_service, expense::build_expense_service,
        governance::build_governance_service, group::build_group_service,
        treasury::build_treasury_service, users::build_user_service,
    },
    state::AppState,
};

use crate::{
    // infrastructure
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtService},
        db::{
            config::DbConfig,
            pool::{DbPool, create_pool},
            repositories::{
                auth_repo_impl::DieselAuthRepository, currency_repo_impl::DieselCurrencyRepository,
                expense_repo_impl::DieselExpenseRepository,
                governance_repo_impl::DieselGovernanceRepository,
                group_repo_impl::DieselGroupRepository,
                group_wallet_repo_impl::DieselGroupWalletRepository,
                transaction_repo_impl::DieselTransactionRepository,
                user_repo_impl::DieselUserRepository,
                user_wallet_repo_impl::DieselUserWalletRepository,
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
    let user_wallet_repo = Arc::new(DieselUserWalletRepository::new(pool.clone()));
    let group_wallet_repo = Arc::new(DieselGroupWalletRepository::new(pool.clone()));
    let transaction_repo = Arc::new(DieselTransactionRepository::new(pool.clone()));
    let currency_repo = Arc::new(DieselCurrencyRepository::new(pool.clone()));
    let governance_repo = Arc::new(DieselGovernanceRepository::new(pool.clone()));
    let expense_repo = Arc::new(DieselExpenseRepository::new(pool.clone()));

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

    let treasury_service = build_treasury_service(
        user_wallet_repo.clone(),
        group_wallet_repo.clone(),
        transaction_repo.clone(),
        currency_repo.clone(),
    );

    let governance_service = build_governance_service(
        governance_repo,
        group_repo.clone(),
        user_repo,
        user_wallet_repo,
    );
    let expense_service = build_expense_service(expense_repo.clone());
    let balances_service = build_balances_service(transaction_repo, group_repo, expense_repo);

    // -------------------------
    // 5. State
    // -------------------------
    let state = Arc::new(AppState {
        config: app_config,

        // Services:
        auth_service,
        user_service,
        group_service,
        treasury_service,
        governance_service,
        expense_service,
        balances_service,
    });

    // -------------------------
    // 6. Router
    // -------------------------
    create_router(state)
}
