use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerificationInput {
    pub email: String,
    pub address: String,
    pub nonce: String,
    pub signature: String,
}

#[derive(Serialize)]
pub struct VerificationOutput {
    pub token: String,
    pub user_id: String,
}
