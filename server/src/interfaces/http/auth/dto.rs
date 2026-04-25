use serde::Deserialize;
use serde::Serialize;

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

// ========= Me =========

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
