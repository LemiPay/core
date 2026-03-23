use crate::data::config::DbConfig;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User id
    pub exp: usize, // Expiration time
}

pub fn generate_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = DbConfig::from_env().jwt_secret;

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60 * 60 * 24 * 7; // 7 days

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
