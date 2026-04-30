use crate::application::group::dto::UserInGroupDetails;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub struct LeaveGroupInput {
    pub group_id: GroupId,
    pub user_id: UserId,
}

pub struct LeaveGroupOutput {
    pub relation: UserInGroupDetails,
}
