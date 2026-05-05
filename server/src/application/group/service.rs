use super::{
    create_group::CreateGroupUseCase, delete_group::DeleteGroupUseCase, get_group::GetGroupUseCase,
    get_group_members::GetGroupMembersUseCase, leave_group::LeaveGroupUseCase,
    list_user_groups::ListUserGroupsUseCase, make_group_admin::MakeGroupAdminUseCase,
    update_group::UpdateGroupUseCase,
};

pub struct GroupService {
    pub create_group: CreateGroupUseCase,
    pub get_group: GetGroupUseCase,
    pub leave_group: LeaveGroupUseCase,
    pub list_user_groups: ListUserGroupsUseCase,
    pub make_group_admin: MakeGroupAdminUseCase,
    pub update_group: UpdateGroupUseCase,
    pub delete_group: DeleteGroupUseCase,
    pub get_group_members: GetGroupMembersUseCase,
}
