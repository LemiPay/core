use serde::Serialize;

// ========= Get User =========

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
