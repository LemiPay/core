use crate::application::group::dto::GroupFromUserDetails;
use crate::domain::user::UserId;

pub struct ListUserGroupsInput {
    pub user_id: UserId,
}

pub struct ListUserGroupsOutput {
    pub groups: Vec<GroupFromUserDetails>,
}
