use crate::data::error::DbError;
use crate::models::proposal::Proposal;
use uuid::Uuid;

pub trait ProposalRepository: Send + Sync {
    fn find_by_group(&self, group_id: Uuid) -> Result<Option<Vec<Proposal>>, DbError>;
    fn find_my_proposals(&self, created_by: Uuid) -> Result<Option<Vec<Proposal>>, DbError>;
}
