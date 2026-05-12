use super::{login::LoginUseCase, register::RegisterUseCase};
use crate::application::auth::challenge::ChallengeUseCase;
use crate::application::auth::verify_challenge::VerifyChallengeUseCase;

pub struct AuthService {
    pub login: LoginUseCase,
    pub register: RegisterUseCase,
    pub challenge: ChallengeUseCase,
    pub verify_challenge: VerifyChallengeUseCase,
}
