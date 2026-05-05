use crate::{application::group::dto::GroupDetails, domain::group::GroupId};

pub struct GetGroupInput {
    pub group_id: GroupId,
}

pub struct GetGroupOutput(pub Option<GroupDetails>);
