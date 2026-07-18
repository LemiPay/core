use std::sync::Arc;

use crate::infrastructure::ai::ai_provider::AiProvider;

use super::context;
use super::dto::ChatMessage;
use super::error::AiError;

pub struct AiService {
    pub provider: Arc<dyn AiProvider>,
}

impl AiService {
    pub fn new(provider: Arc<dyn AiProvider>) -> Self {
        Self { provider }
    }

    pub async fn ask(
        &self,
        system_prompt: &str,
        context: &str,
        question: &str,
        history: &[ChatMessage],
    ) -> Result<String, AiError> {
        self.provider
            .chat(system_prompt, context, question, history)
            .await
            .map_err(Into::into)
    }

    pub async fn explain(&self, concept: &str) -> Result<String, AiError> {
        let system_prompt = context::explain_system_prompt(concept);
        self.provider
            .chat(&system_prompt, "", context::EXPLAIN_QUESTION, &[])
            .await
            .map_err(Into::into)
    }
}
