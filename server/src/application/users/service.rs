use crate::application::users::get_user::UserUseCase;

pub struct UserService {
    pub get_user: UserUseCase,
}
