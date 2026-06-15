use std::sync::Arc;

use crate::application::balances::BalancesService;
use crate::application::group::enter_debt_resolution::EnterDebtResolutionUseCase;

use crate::{
    application::group::{
        GroupService, create_group::CreateGroupUseCase, delete_group::DeleteGroupUseCase,
        get_group::GetGroupUseCase, get_group_members::GetGroupMembersUseCase,
        leave_group::LeaveGroupUseCase, list_user_groups::ListUserGroupsUseCase,
        make_group_admin::MakeGroupAdminUseCase, update_group::UpdateGroupUseCase,
    },
    infrastructure::db::repositories::{
        governance_repo_impl::DieselGovernanceRepository, group_repo_impl::DieselGroupRepository,
        investment_repo_impl::DieselInvestmentRepository,
        notifications_repo_impl::DieselNotificationRepository,
    },
};

pub fn build_group_service(
    group_repo: Arc<DieselGroupRepository>,
    investment_repo: Arc<DieselInvestmentRepository>,
    governance_repo: Arc<DieselGovernanceRepository>,
    notification_repo: Arc<DieselNotificationRepository>,
    balances_service: BalancesService,
) -> GroupService {
    GroupService {
        create_group: CreateGroupUseCase {
            group_repo: group_repo.clone(),
            notification_repo,
        },
        get_group: GetGroupUseCase {
            group_repo: group_repo.clone(),
        },
        leave_group: LeaveGroupUseCase {
            group_repo: group_repo.clone(),
            balances_service: balances_service.clone(),
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
            balances_service: balances_service.clone(),
        },
        get_group_members: GetGroupMembersUseCase {
            group_repo: group_repo.clone(),
        },
        enter_debt_resolution: EnterDebtResolutionUseCase {
            group_repo: group_repo.clone(),
            investment_repo: investment_repo.clone(),
            governance_repo: governance_repo.clone(),
            balances_service: balances_service.clone(),
        },
    }
}
