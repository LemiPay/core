use crate::domain::user::UserId;

pub struct AskInput {
    pub user_id: UserId,
    pub question: String,
}

pub struct AskOutput {
    pub answer: String,
}

pub struct ExplainOutput {
    pub explanation: String,
}
