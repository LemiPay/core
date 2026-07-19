use axum::serve;
use tokio::net::TcpListener;

use server::infrastructure::env;
use server::setup::app_builder::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();

    // 🚀 Server
    use std::net::SocketAddr;
    let addr = env::get_address();

    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
