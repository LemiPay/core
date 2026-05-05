use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub struct CreateGroupInput {
    pub name: String,
    pub description: String,
    pub creator_id: UserId,
}

pub struct CreateGroupOutput {
    pub group_id: GroupId,
}
