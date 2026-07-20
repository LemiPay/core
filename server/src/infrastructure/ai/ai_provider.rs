use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::application::ai::dto::ChatMessage;

const MIN_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
const MAX_RETRIES: u32 = 3;
const RETRY_DELAYS: [u64; 3] = [5, 15, 30];

#[derive(Debug)]
pub enum AiProviderError {
    ApiError(String),
    RateLimited,
    Internal,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn chat(
        &self,
        system_prompt: &str,
        context: &str,
        question: &str,
        history: &[ChatMessage],
    ) -> Result<String, AiProviderError>;
}

pub struct GeminiProvider {
    api_key: String,
    client: reqwest::Client,
    model: String,
    base_url: String,
    last_request: Arc<Mutex<tokio::time::Instant>>,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model,
            base_url,
            last_request: Arc::new(Mutex::new(
                tokio::time::Instant::now() - std::time::Duration::from_secs(10),
            )),
        }
    }

    async fn throttle(&self) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < MIN_INTERVAL {
            tokio::time::sleep(MIN_INTERVAL - elapsed).await;
        }
        *last = tokio::time::Instant::now();
    }

    fn build_contents(
        history: &[ChatMessage],
        context: &str,
        question: &str,
    ) -> Vec<serde_json::Value> {
        let mut contents: Vec<serde_json::Value> = Vec::new();
        for msg in history {
            let role = match msg.role.as_str() {
                "assistant" => "model",
                _ => "user",
            };
            contents.push(serde_json::json!({
                "role": role,
                "parts": [{ "text": msg.content }]
            }));
        }
        let current = format!("{}\n\nQuestion: {}", context, question);
        contents.push(serde_json::json!({
            "role": "user",
            "parts": [{ "text": current }]
        }));
        contents
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn chat(
        &self,
        system_prompt: &str,
        context: &str,
        question: &str,
        history: &[ChatMessage],
    ) -> Result<String, AiProviderError> {
        let url = format!(
            "{}/{}:generateContent?key={}",
            self.base_url.trim_end_matches('/'),
            self.model,
            self.api_key
        );

        let body = serde_json::json!({
            "system_instruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": Self::build_contents(history, context, question)
        });

        let mut last_error = AiProviderError::Internal;

        for attempt in 0..=MAX_RETRIES {
            self.throttle().await;

            let res = self
                .client
                .post(&url)
                .json(&body)
                .send()
                .await
                .map_err(|e| AiProviderError::ApiError(e.to_string()))?;

            if res.status().is_success() {
                let data: serde_json::Value = res
                    .json()
                    .await
                    .map_err(|e| AiProviderError::ApiError(e.to_string()))?;

                return data["candidates"][0]["content"]["parts"][0]["text"]
                    .as_str()
                    .ok_or(AiProviderError::Internal)
                    .map(|s| s.to_string());
            }

            let status = res.status();
            if status == 429 && attempt < MAX_RETRIES {
                last_error = AiProviderError::RateLimited;
                tokio::time::sleep(std::time::Duration::from_secs(
                    RETRY_DELAYS[attempt as usize],
                ))
                .await;
                continue;
            }

            return if status == 429 {
                Err(AiProviderError::RateLimited)
            } else {
                let body_text = res.text().await.unwrap_or_default();
                Err(AiProviderError::ApiError(format!(
                    "Status: {} - {}",
                    status, body_text
                )))
            };
        }

        Err(last_error)
    }
}
