use crate::domain::user::User;

pub struct StoredUser {
    pub user: User,
    pub password_hash: String,
}
