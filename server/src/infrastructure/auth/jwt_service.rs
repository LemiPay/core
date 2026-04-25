use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::auth::{
    jwt_token::JwtToken,
    token_service::{TokenError, TokenService},
};

use crate::domain::user::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User id
    pub exp: usize,  // Expiration time
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenService for JwtService {
    ///
    /// Generate a JWT for a given user ID. The token will expire in 7 days.
    ///
    fn generate(&self, user_id: UserId) -> Result<JwtToken, TokenError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60 * 60 * 24 * 7; // 7 days

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| TokenError::Internal)?;

        Ok(JwtToken(token))
    }

    ///
    /// Decode a JWT and return the claims if valid.
    /// Returns an error if the token is invalid or expired.
    ///
    fn verify(&self, token: &str) -> Result<UserId, TokenError> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| TokenError::Invalid)?;

        let user_id = uuid::Uuid::parse_str(&data.claims.sub).map_err(|_| TokenError::Invalid)?;

        Ok(UserId(user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn test_service() -> JwtService {
        JwtService::new("test_secret".to_string())
    }

    #[test]
    fn should_generate_and_verify_token() {
        let service = test_service();

        let user_id = UserId(Uuid::new_v4());

        let token = service.generate(user_id).unwrap();

        let result = service.verify(&token.0).unwrap();

        assert_eq!(result, user_id);
    }

    #[test]
    fn should_fail_with_invalid_token() {
        let service = test_service();

        let result = service.verify("invalid.token.here");

        assert!(result.is_err());
    }

    #[test]
    fn should_fail_with_wrong_secret() {
        let service1 = test_service();
        let service2 = JwtService::new("secret2".to_string());

        let user_id = UserId(Uuid::new_v4());

        let token = service1.generate(user_id).unwrap();

        let result = service2.verify(&token.0);

        assert!(result.is_err());
    }

    #[test]
    fn should_fail_if_token_expired() {
        let service = test_service();

        let expired_claims = Claims {
            sub: Uuid::new_v4().to_string(),
            exp: 0, // 👈 ya expirado
        };

        let token = encode(
            &Header::default(),
            &expired_claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();

        let result = service.verify(&token);

        assert!(result.is_err());
    }
}
