use crate::data::error::DbError;
use crate::models::group::{Group, GroupUpdate};
use crate::models::user_in_group::{GroupFromUser, GroupMember, UserInGroup};
use uuid::Uuid;

pub trait GroupRepository: Send + Sync {
    fn create_group(
        &self,
        name: String,
        description: String,
        user_id: Uuid,
    ) -> Result<Group, DbError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DbError>;
    fn make_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError>;
    fn add_user_to_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError>;
    fn delete_group(&self, group_id: Uuid) -> Result<Group, DbError>;
    fn is_group_active(&self, group_id: Uuid) -> Result<bool, DbError>;
    fn get_group_members(&self, group_id: Uuid) -> Result<Vec<GroupMember>, DbError>;
    fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<GroupFromUser>, DbError>;
    fn update_group_info(
        &self,
        group_id: Uuid,
        group_update: GroupUpdate,
    ) -> Result<Group, DbError>;

    // Predicates
    fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError>;
    fn is_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError>;
}
