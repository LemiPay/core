// infrastructure/auth/argon2_hasher.rs

use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand::rngs::OsRng;

use crate::application::auth::password_hasher::{HashError, PasswordHasher as PasswordHasherTrait};

pub struct Argon2Hasher {
    argon2: Argon2<'static>,
}

impl Argon2Hasher {
    pub fn new() -> Result<Self, Error> {
        let params = {
            #[cfg(test)]
            {
                Params::new(4096, 1, 1, None)?
            }

            #[cfg(not(test))]
            {
                Params::new(65536, 3, 1, None)?
            }
        };

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        Ok(Self { argon2 })
    }
}

impl PasswordHasherTrait for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, HashError> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| HashError::HashFailed)?;

        Ok(password_hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, HashError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| HashError::VerifyFailed)?;

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|_| HashError::VerifyFailed)
    }
}

// === Tests ===

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hash_and_verify_password() {
        let hasher = Argon2Hasher::new().unwrap();

        let password = "super_secret";

        let hash = hasher.hash(password).unwrap();

        let valid = hasher.verify(password, &hash).unwrap();

        assert!(valid);
    }

    #[test]
    fn should_fail_with_wrong_password() {
        let hasher = Argon2Hasher::new().unwrap();

        let hash = hasher.hash("correct_password").unwrap();

        let valid = hasher.verify("wrong_password", &hash).unwrap();

        assert!(!valid);
    }

    #[test]
    fn should_generate_different_hashes_for_same_password() {
        let hasher = Argon2Hasher::new().unwrap();

        let password = "same_password";

        let hash1 = hasher.hash(password).unwrap();
        let hash2 = hasher.hash(password).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn should_fail_with_invalid_hash_format() {
        let hasher = Argon2Hasher::new().unwrap();

        let result = hasher.verify("password", "not_a_valid_hash");

        assert!(result.is_err());
    }
}
