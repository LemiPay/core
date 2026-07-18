use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct AskOutput {
    pub answer: String,
}

pub struct ExplainOutput {
    pub explanation: String,
}
