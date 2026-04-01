use crate::data::error::DbError;
use crate::models::group::Group;
use crate::models::user_in_group::UserInGroup;
use uuid::Uuid;

pub trait GroupRepository: Send + Sync {
    fn create_group(
        &self,
        name: String,
        description: String,
        user_id: Uuid,
    ) -> Result<Group, DbError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DbError>;
    fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError>;
    fn is_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError>;
    fn make_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError>;
    fn add_user_to_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError>;
}
