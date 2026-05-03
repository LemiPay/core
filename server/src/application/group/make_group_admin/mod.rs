use std::sync::Arc;

use crate::{
    application::group::{dto::UserInGroupDetails, traits::repository::GroupRepository},
    domain::{group::GroupId, user::UserId},
};

#[derive(Debug)]
pub enum MakeGroupAdminError {
    Forbidden,
    NotFound,
    BadRequest(String),
    Internal,
}

#[derive(Clone)]
pub struct MakeGroupAdminUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl MakeGroupAdminUseCase {
    pub fn execute(
        &self,
        actor_id: UserId,
        group_id: GroupId,
        new_admin_id: UserId,
    ) -> Result<UserInGroupDetails, MakeGroupAdminError> {
        if !self
            .group_repo
            .is_admin(actor_id, group_id)
            .map_err(|_| MakeGroupAdminError::Internal)?
        {
            return Err(MakeGroupAdminError::Forbidden);
        }

        let group = self
            .group_repo
            .find_by_id(group_id)
            .map_err(|_| MakeGroupAdminError::Internal)?;
        let group = group.ok_or(MakeGroupAdminError::NotFound)?;

        if !group.has_member(new_admin_id) {
            return Err(MakeGroupAdminError::BadRequest(
                "El usuario no pertenece al grupo".into(),
            ));
        }

        if group.member(new_admin_id).is_some_and(|m| m.is_admin()) {
            return Err(MakeGroupAdminError::BadRequest(
                "El usuario ya es administrador".into(),
            ));
        }

        let updated = group
            .promote_member_to_admin(new_admin_id)
            .map_err(|_| MakeGroupAdminError::Internal)?;

        self.group_repo
            .save(&updated)
            .map_err(|_| MakeGroupAdminError::Internal)?;

        self.group_repo
            .get_user_in_group(new_admin_id, group_id)
            .map_err(|_| MakeGroupAdminError::Internal)?
            .ok_or(MakeGroupAdminError::Internal)
    }
}
