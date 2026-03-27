use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use rand::rngs::OsRng;

fn get_hash_algo() -> Result<Argon2<'static>, argon2::password_hash::Error> {
    let params = Params::new(65536, 3, 1, None)?;

    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

/// 🔐 Hashea una password
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = get_hash_algo()?.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

/// 🔍 Verifica una password contra su hash
pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    get_hash_algo()?.verify_password(password.as_bytes(), &parsed_hash)
}
