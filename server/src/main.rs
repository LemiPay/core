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
use crate::repositories::diesel::group_repo_impl::DieselGroupRepository;
use crate::repositories::diesel::proposal_repo_impl::DieselProposalRepository;
use crate::repositories::diesel::user_repo_impl::DieselUserRepository;
// Services
use crate::services::auth::AuthService;
use crate::services::group::GroupService;
use crate::services::proposal::ProposalService;
use crate::services::user::UserService;

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

    // 🧠 Service
    let user_service = UserService::new(user_repo.clone());
    let auth_service = AuthService::new(auth_repo.clone());
    let group_service = GroupService::new(group_repo.clone());
    let proposal_service = ProposalService::new(proposal_repo.clone(), user_repo.clone());

    let state = Arc::new(AppState {
        db: db.clone(),
        user_service,
        auth_service,
        group_service,
        proposal_service,
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
