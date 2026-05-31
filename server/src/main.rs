use axum::serve;
use std::time::Duration;
use tokio::net::TcpListener;

use server::application::investment::pulse::process_pulse;
use server::setup::app_builder::build_app;

#[tokio::main]
async fn main() {
    let (app, pool) = build_app();

    // Background pulse scheduler — 1 pulse every 10s = 1 simulated day
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            let pool = pool.clone();
            match tokio::task::spawn_blocking(move || process_pulse(&pool)).await {
                Ok(Ok(res)) => {
                    println!("Pulse: {} updated, {} matured", res.updated, res.matured)
                }
                Ok(Err(e)) => eprintln!("Pulse error: {}", e),
                Err(e) => eprintln!("Pulse task panicked: {}", e),
            }
        }
    });

    // 🚀 Server
    use std::net::SocketAddr;
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();

    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
