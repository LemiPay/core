use super::types::*;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: Email,
}
