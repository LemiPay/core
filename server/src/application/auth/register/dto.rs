use crate::domain::user::UserId;

pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterOutput {
    pub user_id: UserId,
}
