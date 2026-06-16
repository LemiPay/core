use std::sync::Arc;

use crate::application::permission::PermissionService;
use crate::infrastructure::db::repositories::{
    group_repo_impl::DieselGroupRepository, permission_repo_impl::DieselPermissionRepository,
};

pub fn build_permission_service(
    permission_repo: Arc<DieselPermissionRepository>,
    group_repo: Arc<DieselGroupRepository>,
) -> PermissionService {
    PermissionService {
        permission_repo,
        group_repo,
    }
}
