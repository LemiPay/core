use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::domain::permission::action::Action;
use crate::infrastructure::db::models::group::GroupRoleModel;
use crate::infrastructure::db::schema;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::group_permission)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupPermissionModel {
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub action: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::group_permission)]
pub struct NewGroupPermissionModel {
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub action: String,
}

impl From<&GroupPermissionModel>
    for (
        crate::domain::group::GroupId,
        crate::domain::group::member::GroupRole,
        Action,
    )
{
    fn from(p: &GroupPermissionModel) -> Self {
        use crate::domain::group::GroupId;
        use crate::domain::group::member::GroupRole;
        let role = match p.role {
            GroupRoleModel::Admin => GroupRole::Admin,
            GroupRoleModel::Member => GroupRole::Member,
        };
        let action = Action::from_name(&p.action)
            .unwrap_or_else(|| panic!("Unknown action in DB: {}", p.action));
        (GroupId(p.group_id), role, action)
    }
}
