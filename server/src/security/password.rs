use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand::rngs::OsRng;

/// 🔐 Hashea una password
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let params = Params::new(
        65536, // memory cost (KB) → 64 MB
        3,     // iterations
        1,     // parallelism
        None,  // output length (default)
    )?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

/// 🔍 Verifica una password contra su hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
