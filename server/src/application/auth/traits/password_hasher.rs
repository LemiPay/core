pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, HashError>;

    fn verify(&self, password: &str, hash: &str) -> Result<bool, HashError>;
}

#[derive(Debug)]
pub enum HashError {
    HashFailed,
    VerifyFailed,
}
