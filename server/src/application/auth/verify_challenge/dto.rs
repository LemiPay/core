use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VerificationInput {
    pub email: Option<String>,
    pub name: Option<String>,
    pub allow_linking: bool,
    pub address: String,
    pub nonce: String,
    pub signature: String,
    /// Si el cache in-memory no tiene el challenge (otra réplica), se usa este valor.
    pub issued_at: Option<String>,
}

#[derive(Serialize)]
pub struct VerificationOutput {
    pub token: String,
    pub user_id: String,
}
