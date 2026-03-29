use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::proposal::Proposal;
use crate::repositories::traits::proposal_repo::ProposalRepository;
use uuid::Uuid;

pub struct DieselProposalRepository {
    db: Db,
}

impl DieselProposalRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl ProposalRepository for DieselProposalRepository {
    fn find_by_group(&self, group_id: Uuid) -> Result<Option<Vec<Proposal>>, DbError> {
        todo!()
    }

    fn find_my_proposals(&self, created_by: Uuid) -> Result<Option<Vec<Proposal>>, DbError> {
        todo!()
    }
}
