use std::sync::Arc;

use crate::application::ai::service::AiService;
use crate::infrastructure::ai::ai_provider::GeminiProvider;
use crate::infrastructure::ai::config::AiConfig;

pub fn build_ai_service(config: AiConfig) -> AiService {
    let provider = Arc::new(GeminiProvider::new(
        config.gemini_api_key,
        config.gemini_model,
        config.gemini_api_url,
    ));

    AiService::new(provider)
}
