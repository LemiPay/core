use std::env;

pub struct AiConfig {
    pub gemini_api_key: String,
    pub gemini_model: String,
    pub gemini_api_url: String,
}

impl AiConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename("../.env").ok();

        Self {
            gemini_api_key: env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
            gemini_model: env::var("GEMINI_MODEL")
                .unwrap_or_else(|_| "gemini-3.5-flash".to_string()),
            gemini_api_url: env::var("GEMINI_API_URL").unwrap_or_else(|_| {
                "https://generativelanguage.googleapis.com/v1beta/models".to_string()
            }),
        }
    }
}
