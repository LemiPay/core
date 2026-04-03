use diesel::{
    Connection, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use uuid::Uuid;

use crate::repositories::traits::proposal_repo::ProposalRepository;

use crate::data::database::Db;
use crate::data::error::DbError;
// Models
use crate::models::proposal::{NewProposal, Proposal, ProposalType, ProposalUpdate};
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

    fn find_new_member_received_by(
        &self,
        destination: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = new_member_proposal::table
            .inner_join(proposal::table.on(nmp::proposal_id.eq(p::id)))
            .filter(nmp::new_member_id.eq(destination))
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

    fn find(&self, proposal_id: Uuid) -> Result<Option<Proposal>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = proposal::table
            .filter(proposal::id.eq(proposal_id))
            .first::<Proposal>(&mut conn)
            .optional()?;
        Ok(result)
    }

    fn create_new_member_proposal(
        &self,
        new_proposal: NewProposal,
        new_user_id: Uuid,
    ) -> Result<NewMemberProposalExpanded, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = conn.transaction::<NewMemberProposalExpanded, DbError, _>(|this_conn| {
            let proposal = diesel::insert_into(proposal::table)
                .values(&new_proposal)
                .returning(Proposal::as_returning())
                .get_result(this_conn)?;

            let params = NewMemberProposal {
                proposal_id: proposal.id,
                new_member_id: new_user_id,
            };

            let new_member_proposal = diesel::insert_into(new_member_proposal::table)
                .values(&params)
                .returning(NewMemberProposal::as_returning())
                .get_result(this_conn)?;

            Ok(NewMemberProposalExpanded {
                proposal,
                new_member_proposal,
                proposal_type: ProposalType::NewMember,
            })
        })?;

        Ok(result)
    }

    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        params: ProposalUpdate,
    ) -> Result<Proposal, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = diesel::update(proposal::table.filter(proposal::id.eq(proposal_id)))
            .set(params)
            .get_result::<Proposal>(&mut conn)?;

        Ok(result)
    }
}
