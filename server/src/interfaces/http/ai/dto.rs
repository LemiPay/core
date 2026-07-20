use serde::{Deserialize, Serialize};

use crate::application::ai::dto::ChatMessage;

#[derive(Deserialize)]
pub struct AskRequest {
    pub question: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
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
