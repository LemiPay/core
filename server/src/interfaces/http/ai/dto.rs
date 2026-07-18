use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AskRequest {
    pub question: String,
}

#[derive(Serialize)]
pub struct AskResponse {
    pub answer: String,
}

#[derive(Deserialize)]
pub struct ExplainRequest {
    pub concept: String,
}

#[derive(Serialize)]
pub struct ExplainResponse {
    pub explanation: String,
}
