use axum::serve;
use std::sync::Arc;

mod data;
pub mod errors;
mod handlers;
pub mod helpers;
mod models;
mod repositories;
mod router;
pub mod routes;
mod schema;
pub mod security;
mod services;

use crate::router::create_router;

use crate::data::config::DbConfig;
use crate::data::database::Db;
use crate::data::state::AppState;

// Repos
use crate::repositories::diesel::auth_repo_impl::DieselAuthRepository;
use crate::repositories::diesel::currency_repo_impl::DieselCurrencyRepository;
use crate::repositories::diesel::expense_repo_impl::DieselExpenseRepository;
use crate::repositories::diesel::fund_round_repo_impl::DieselFundRoundRepository;
use crate::repositories::diesel::group_repo_impl::DieselGroupRepository;
use crate::repositories::diesel::group_wallet_repo_impl::DieselGroupWalletRepository;
use crate::repositories::diesel::proposal_repo_impl::DieselProposalRepository;
use crate::repositories::diesel::transaction_repo_impl::DieselTransactionRepository;
use crate::repositories::diesel::user_repo_impl::DieselUserRepository;
use crate::repositories::diesel::user_wallet_repo_impl::DieselUserWalletRepository;

// Services
use crate::services::auth::AuthService;
use crate::services::expense::ExpenseService;
use crate::services::group::GroupService;
use crate::services::group_wallet::GroupWalletService;
use crate::services::proposal::ProposalService;
use crate::services::transaction::TransactionService;
use crate::services::user::UserService;
use crate::services::user_wallet::UserWalletService;

#[tokio::main]
async fn main() {
    // 🔧 Config
    let db_config = DbConfig::from_env();

    // 🗄️ DB
    let db = Db::new(db_config);

    // 📦 Repository
    let user_repo = Arc::new(DieselUserRepository::new(db.clone()));
    let auth_repo = Arc::new(DieselAuthRepository::new(db.clone()));
    let group_repo = Arc::new(DieselGroupRepository::new(db.clone()));
    let proposal_repo = Arc::new(DieselProposalRepository::new(db.clone()));
    let transaction_repo = Arc::new(DieselTransactionRepository::new(db.clone()));
    let user_wallet_repo = Arc::new(DieselUserWalletRepository::new(db.clone()));
    let currency_repo = Arc::new(DieselCurrencyRepository::new(db.clone()));
    let fund_round_repo = Arc::new(DieselFundRoundRepository::new(db.clone()));
    let group_wallet_repo = Arc::new(DieselGroupWalletRepository::new(db.clone()));
    let expense_repo = Arc::new(DieselExpenseRepository::new(db.clone()));

    // 🧠 Service
    let user_service = UserService::new(user_repo.clone());
    let auth_service = AuthService::new(auth_repo.clone());
    let group_service = GroupService::new(group_repo.clone());
    let proposal_service =
        ProposalService::new(proposal_repo.clone(), user_repo.clone(), group_repo.clone());
    let transaction_service =
        TransactionService::new(transaction_repo.clone(), proposal_repo.clone());
    let user_wallet_service =
        UserWalletService::new(user_wallet_repo.clone(), currency_repo.clone());
    let group_wallet_service = GroupWalletService::new(
        fund_round_repo.clone(),
        currency_repo.clone(),
        group_repo.clone(),
        group_wallet_repo.clone(),
        proposal_repo.clone(),
        user_wallet_repo.clone(),
    );
    let expense_service = ExpenseService::new(expense_repo.clone());
    let state = Arc::new(AppState {
        user_service,
        auth_service,
        group_service,
        proposal_service,
        transaction_service,
        user_wallet_service,
        group_wallet_service,
        expense_service,
    });

    // 🚏 Router
    let app = create_router(state);

    // 🚀 Server
    use std::net::SocketAddr;
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();

    println!("Server running on http://{}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
