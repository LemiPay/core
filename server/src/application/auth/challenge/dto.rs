pub struct ChallengeOutput {
    pub nonce: String,
    pub message: String,
    pub is_linked: bool,
}

pub struct ChallengeInput {
    pub address: String,
}
