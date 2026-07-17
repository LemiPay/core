use async_trait::async_trait;

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
    ) -> Result<String, AiProviderError>;
}

pub struct GeminiProvider {
    api_key: String,
    client: reqwest::Client,
    model: String,
    base_url: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model,
            base_url,
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn chat(
        &self,
        system_prompt: &str,
        context: &str,
        question: &str,
    ) -> Result<String, AiProviderError> {
        let url = format!(
            "{}/{}:generateContent?key={}",
            self.base_url.trim_end_matches('/'),
            self.model,
            self.api_key
        );

        let content = format!("{}\n\nQuestion: {}", context, question);

        let body = serde_json::json!({
            "system_instruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": [
                { "role": "user", "parts": [{ "text": content }] }
            ]
        });

        let res = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AiProviderError::ApiError(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
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

        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| AiProviderError::ApiError(e.to_string()))?;

        let text = data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or(AiProviderError::Internal)?
            .to_string();

        Ok(text)
    }
}
