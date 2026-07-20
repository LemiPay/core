use crate::infrastructure::ai::ai_provider::AiProviderError;

#[derive(Debug)]
pub enum AiError {
    Provider(String),
    RateLimited,
    Internal,
}

impl From<AiProviderError> for AiError {
    fn from(e: AiProviderError) -> Self {
        match e {
            AiProviderError::ApiError(msg) => AiError::Provider(msg),
            AiProviderError::RateLimited => AiError::RateLimited,
            AiProviderError::Internal => AiError::Internal,
        }
    }
}
