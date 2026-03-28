use crate::data::error::DbError;
use crate::models::group::Group;
use uuid::Uuid;

pub trait GroupRepository: Send + Sync {
    fn create_group(
        &self,
        name: String,
        description: String,
        user_id: Uuid,
    ) -> Result<Group, DbError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DbError>;
}
