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
}

#[derive(Deserialize)]
pub struct VerificationRequest {
    pub email: String,
    pub address: String,
    pub nonce: String,
    pub signature: String,
}
#[derive(Serialize)]
pub struct VerificationResponse {
    pub token: String,
    pub user_id: String,
}
