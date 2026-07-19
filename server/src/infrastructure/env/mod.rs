use std::net::SocketAddr;

pub fn get_address() -> SocketAddr {
    dotenvy::dotenv().ok();
    dotenvy::from_filename("../.env").ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    format!("0.0.0.0:{}", port).parse().unwrap()
}
