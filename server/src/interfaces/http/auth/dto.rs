use serde::{Deserialize, Serialize};

// ========= Register =========

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
}

// ========= Login =========

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

// ====== ReOwn Login

#[derive(Deserialize)]
pub struct ChallengeRequest {
    pub address: String,
}

#[derive(Serialize)]
pub struct ChallengeResponse {
    pub nonce: String,
    pub message: String,
    pub is_linked: bool,
    pub issued_at: String,
}

#[derive(Deserialize)]
pub struct VerificationRequest {
    pub email: Option<String>,
    pub name: Option<String>,
    pub allow_linking: Option<bool>,
    pub address: String,
    pub nonce: String,
    pub signature: String,
    /// Debe coincidir con el challenge; permite verify sin cache (multi-réplica).
    pub issued_at: Option<String>,
}
#[derive(Serialize)]
pub struct VerificationResponse {
    pub token: String,
    pub user_id: String,
}
