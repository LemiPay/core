use std::sync::Arc;

use validator::ValidateLength;

use crate::{
    application::group::{dto::GroupDetails, traits::repository::GroupRepository},
    domain::{group::GroupId, user::UserId},
};

#[derive(Debug)]
pub enum UpdateGroupError {
    Forbidden,
    NotFound,
    BadRequest(String),
    Internal,
}

#[derive(Clone)]
pub struct UpdateGroupUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl UpdateGroupUseCase {
    pub fn execute(
        &self,
        actor_id: UserId,
        group_id: GroupId,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<GroupDetails, UpdateGroupError> {
        if !self
            .group_repo
            .is_admin(actor_id, group_id)
            .map_err(|_| UpdateGroupError::Internal)?
        {
            return Err(UpdateGroupError::Forbidden);
        }

        if name.is_none() && description.is_none() {
            return Err(UpdateGroupError::BadRequest(
                "No hay campos para actualizar".into(),
            ));
        }

        if let Some(ref n) = name
            && !ValidateLength::validate_length(n.trim(), Some(4), Some(30), None)
        {
            return Err(UpdateGroupError::BadRequest(
                "Nombre de grupo inválido: debe tener entre 4 y 30 caracteres".into(),
            ));
        }

        if let Some(ref d) = description
            && !ValidateLength::validate_length(d.trim(), Some(8), Some(30), None)
        {
            return Err(UpdateGroupError::BadRequest(
                "Descripción de grupo inválida: debe tener entre 8 y 30 caracteres".into(),
            ));
        }

        let group = self
            .group_repo
            .find_by_id(group_id)
            .map_err(|_| UpdateGroupError::Internal)?
            .ok_or(UpdateGroupError::NotFound)?;
        let updated = group
            .update_info(name, description)
            .map_err(|_| UpdateGroupError::Internal)?;

        self.group_repo
            .save(&updated)
            .map_err(|_| UpdateGroupError::Internal)?;

        self.group_repo
            .get_group_details(group_id)
            .map_err(|_| UpdateGroupError::Internal)?
            .ok_or(UpdateGroupError::NotFound)
    }
}
