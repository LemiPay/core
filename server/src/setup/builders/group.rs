use std::sync::Arc;

use crate::{
    application::group::{
        GroupService, create_group::CreateGroupUseCase, delete_group::DeleteGroupUseCase,
        get_group::GetGroupUseCase, get_group_members::GetGroupMembersUseCase,
        leave_group::LeaveGroupUseCase, list_user_groups::ListUserGroupsUseCase,
        make_group_admin::MakeGroupAdminUseCase, update_group::UpdateGroupUseCase,
    },
    infrastructure::db::repositories::group_repo_impl::DieselGroupRepository,
};

pub fn build_group_service(group_repo: Arc<DieselGroupRepository>) -> GroupService {
    GroupService {
        create_group: CreateGroupUseCase {
            group_repo: group_repo.clone(),
        },
        get_group: GetGroupUseCase {
            group_repo: group_repo.clone(),
        },
        leave_group: LeaveGroupUseCase {
            group_repo: group_repo.clone(),
        },
        list_user_groups: ListUserGroupsUseCase {
            group_repo: group_repo.clone(),
        },
        make_group_admin: MakeGroupAdminUseCase {
            group_repo: group_repo.clone(),
        },
        update_group: UpdateGroupUseCase {
            group_repo: group_repo.clone(),
        },
        delete_group: DeleteGroupUseCase {
            group_repo: group_repo.clone(),
        },
        get_group_members: GetGroupMembersUseCase { group_repo },
    }
}
