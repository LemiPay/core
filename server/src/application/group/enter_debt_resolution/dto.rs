use crate::application::group::dto::GroupDetails;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub struct EnterDebtResolutionInput {
    pub group_id: GroupId,
    pub actor_id: UserId,
}

pub struct EnterDebtResolutionOutput {
    pub group: GroupDetails,
}
