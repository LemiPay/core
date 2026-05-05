use crate::application::auth::challenge::ChallengeUseCase;
use super::{login::LoginUseCase, register::RegisterUseCase};

pub struct AuthService {
    pub login: LoginUseCase,
    pub register: RegisterUseCase,
    pub challenge: ChallengeUseCase
}
