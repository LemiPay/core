use crate::data::error::DbError;
use crate::models::user_in_group;
use crate::models::user_in_group::UserInGroup;
use uuid::Uuid;

pub trait UserInGroupRepo: Send + Sync {
    fn add_user_to_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError>;
}
