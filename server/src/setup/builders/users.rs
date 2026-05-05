use std::sync::Arc;

use crate::application::users::{UserService, get_user::UserUseCase};

use crate::infrastructure::db::repositories::user_repo_impl::DieselUserRepository;

pub fn build_user_service(user_repo: Arc<DieselUserRepository>) -> UserService {
    UserService {
        get_user: UserUseCase { repo: user_repo },
    }
}
