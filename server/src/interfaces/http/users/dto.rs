use serde::Serialize;

// ========= Get User =========

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

// ========= Me =========

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
