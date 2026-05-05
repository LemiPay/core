use std::sync::Arc;

use crate::application::group::get_group::dto::{GetGroupInput, GetGroupOutput};
use crate::application::group::get_group::error::GetGroupError;
use crate::application::group::traits::repository::GroupRepository;

#[derive(Clone)]
pub struct GetGroupUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl GetGroupUseCase {
    pub fn execute(&self, input: GetGroupInput) -> Result<GetGroupOutput, GetGroupError> {
        let group = self
            .group_repo
            .get_group_details(input.group_id)
            .map_err(|_| GetGroupError::InternalError)?;

        Ok(GetGroupOutput(group))
    }
}
