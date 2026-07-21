pub struct ChallengeOutput {
    pub nonce: String,
    pub message: String,
    pub is_linked: bool,
    /// Timestamp RFC3339 embebido en el mensaje; el cliente lo reenvía en verify
    /// para no depender solo del cache in-memory (multi-instancia / races).
    pub issued_at: String,
}

pub struct ChallengeInput {
    pub address: String,
}
