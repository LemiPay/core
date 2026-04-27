use axum::serve;

use server::setup::app_builder::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();

    // 🚀 Server
    use std::net::SocketAddr;
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();

    println!("Server running on http://{}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
