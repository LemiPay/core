use crate::domain::user::{User, UserId};

pub struct GetUserInput {
    pub(crate) user_id: UserId,
}

pub struct GetUserOutput(pub Option<User>);
