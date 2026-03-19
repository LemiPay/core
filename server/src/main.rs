use axum::serve;
use std::sync::Arc;

mod data;
pub mod errors;
mod handlers;
mod models;
mod repositories;
mod router;
pub mod routes;
mod schema;
mod services;

use crate::handlers::user::AppState;
use crate::router::create_router;

use crate::data::config::DbConfig;
use crate::data::database::Db;

use crate::repositories::diesel::user_repo_impl::DieselUserRepository;
use crate::services::user::UserService;

#[tokio::main]
async fn main() {
    // 🔧 Config
    let db_config = DbConfig::from_env();

    // 🗄️ DB
    let db = Db::new(db_config);

    // 📦 Repository
    let user_repo = Arc::new(DieselUserRepository::new(db));

    // 🧠 Service
    let user_service = Arc::new(UserService::new(user_repo));

    // 🌐 App State
    let state = AppState { user_service };

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
