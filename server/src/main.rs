use axum::serve;
use tokio::net::TcpListener;

use server::infrastructure::env;
use server::setup::app_builder::build_app;

#[tokio::main]
async fn main() {
    println!("=== Checkpoint 1 ===");
    let app = build_app();

    println!("=== Checkpoint 2 ===");
    // 🚀 Server
    use std::net::SocketAddr;
    let addr = env::get_address();

    println!("=== Checkpoint 3 ===");
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("=== Checkpoint 3 ===");

    serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
