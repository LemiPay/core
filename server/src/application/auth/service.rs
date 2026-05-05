use super::{login::LoginUseCase, register::RegisterUseCase};
use crate::application::auth::challenge::ChallengeUseCase;

pub struct AuthService {
    pub login: LoginUseCase,
    pub register: RegisterUseCase,
    pub challenge: ChallengeUseCase,
}
