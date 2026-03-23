use crate::data::config::DbConfig;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User id
    pub exp: usize, // Expiration time
}

///
/// Generate a JWT for a given user ID. The token will expire in 7 days.
///
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

///
/// Decode a JWT and return the claims if valid.
/// Returns an error if the token is invalid or expired.
///
pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = DbConfig::from_env().jwt_secret;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
