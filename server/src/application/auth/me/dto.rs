use crate::domain::user::{Email, UserId};

pub struct MeOutput {
    pub user_id: UserId,
    pub email: Email,
    pub name: String,
}
