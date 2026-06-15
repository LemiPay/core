use axum::Router;
use std::sync::Arc;
use std::time::Duration;

// bootstrap
use super::router::create_router;

use crate::setup::{
    builders::{
        auth::build_auth_service, balances::build_balances_service, expense::build_expense_service,
        governance::build_governance_service, group::build_group_service,
        investment::build_investment_service, treasury::build_treasury_service,
        users::build_user_service,
    },
    state::AppState,
};

use crate::infrastructure::auth::web_3_auth::Web3Auth;
use crate::setup::builders::settlements::build_settlements_service;
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
                investment_repo_impl::DieselInvestmentRepository,
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
    let investment_repo = Arc::new(DieselInvestmentRepository::new(pool.clone()));

    let hash_service = Arc::new(Argon2Hasher::new().expect("argon2 fail"));
    let token_service = Arc::new(JwtService::new(db_config.jwt_secret));
    let web_3_service = Arc::new(Web3Auth::new());

    // -------------------------
    // 4. Application Services
    // -------------------------
    let auth_service = build_auth_service(
        auth_repo.clone(),
        user_repo.clone(),
        hash_service,
        token_service,
        web_3_service,
        user_wallet_repo.clone(),
    );

    let user_service = build_user_service(user_repo.clone());

    let treasury_service = build_treasury_service(
        group_repo.clone(),
        user_wallet_repo.clone(),
        group_wallet_repo.clone(),
        transaction_repo.clone(),
        currency_repo.clone(),
    );

    let governance_service = build_governance_service(
        governance_repo,
        group_repo.clone(),
        user_repo,
        user_wallet_repo.clone(),
    );
    let expense_service = build_expense_service(group_repo.clone(), expense_repo.clone());
    let balances_service =
        build_balances_service(transaction_repo.clone(), group_repo.clone(), expense_repo);
    let group_service = build_group_service(
        group_repo.clone(),
        investment_repo.clone(),
        balances_service.clone(),
    );
    let investment_service = build_investment_service(
        investment_repo,
        group_repo.clone(),
        group_wallet_repo.clone(),
        balances_service.clone(),
    );
    let settlements_service = build_settlements_service(
        balances_service.clone(),
        group_repo.clone(),
        user_wallet_repo.clone(),
        group_wallet_repo.clone(),
        transaction_repo.clone(),
    );

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
        settlements_service,
        investment_service,
    });

    // -------------------------
    // 6. Background pulse scheduler — 1 pulse every 10s = 1 simulated day
    // -------------------------
    let pulse_svc = state.investment_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let svc = pulse_svc.clone();
            match tokio::task::spawn_blocking(move || svc.process_pulse()).await {
                Ok(Ok(res)) => {
                    println!("Pulse: {} updated, {} matured", res.updated, res.matured)
                }
                Ok(Err(e)) => eprintln!("Pulse error: {}", e),
                Err(e) => eprintln!("Pulse task panicked: {}", e),
            }
        }
    });

    // -------------------------
    // 7. Router
    // -------------------------
    create_router(state)
}
