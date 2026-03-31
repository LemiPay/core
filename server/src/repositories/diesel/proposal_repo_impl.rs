use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::repositories::traits::proposal_repo::ProposalRepository;

use crate::data::database::Db;
use crate::data::error::DbError;

// Models
use crate::models::proposal::{Proposal, ProposalType};
use crate::models::proposals::new_member::{NewMemberProposal, NewMemberProposalExpanded};

// Schema
use crate::schema::new_member_proposal::dsl as nmp;
use crate::schema::proposal::dsl as p;
use crate::schema::{new_member_proposal, proposal};

pub struct DieselProposalRepository {
    db: Db,
}

impl DieselProposalRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl ProposalRepository for DieselProposalRepository {
    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<NewMemberProposalExpanded>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = new_member_proposal::table
            .inner_join(proposal::table.on(nmp::proposal_id.eq(p::id)))
            .filter(p::group_id.eq(group_id))
            .load::<(NewMemberProposal, Proposal)>(&mut conn)?;

        let parsed: Vec<NewMemberProposalExpanded> = result
            .into_iter()
            .map(|(nmp, p)| NewMemberProposalExpanded {
                proposal: p,
                new_member_proposal: nmp,
                proposal_type: ProposalType::NewMember,
            })
            .collect();

        Ok(parsed)
    }

    fn find_my_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = new_member_proposal::table
            .inner_join(proposal::table.on(nmp::proposal_id.eq(p::id)))
            .filter(p::created_by.eq(created_by))
            .load::<(NewMemberProposal, Proposal)>(&mut conn)?;

        let parsed: Vec<NewMemberProposalExpanded> = result
            .into_iter()
            .map(|(nmp, p)| NewMemberProposalExpanded {
                proposal: p,
                new_member_proposal: nmp,
                proposal_type: ProposalType::NewMember,
            })
            .collect();

        Ok(parsed)
    }
}
