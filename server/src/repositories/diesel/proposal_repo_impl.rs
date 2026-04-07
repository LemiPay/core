use diesel::{
    Connection, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use uuid::Uuid;

use crate::repositories::traits::proposal_repo::ProposalRepository;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::errors::app_error::AppError;
// Models
use crate::models::proposal::{
    MyProposalStatus, NewProposal, Proposal, ProposalType, ProposalUpdate,
};
use crate::models::proposals::new_member::{
    NewMemberProposal, NewMemberProposalExpanded, ReceivedNewMemberProposalExpanded,
};
use crate::models::user_in_group::{MyGroupRole, NewUserInGroup, UserInGroup};
// Schema
use crate::schema::new_member_proposal::dsl as nmp;
use crate::schema::proposal::dsl as p;
use crate::schema::{group, new_member_proposal, proposal, user, user_in_group};

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

    fn respond_to_new_member_proposal(
        &self,
        new_member_proposal_id: Uuid,
        destination: Uuid,
        approve: bool,
    ) -> Result<NewMemberProposalExpanded, AppError> {
        let mut conn = self.db.get_conn()?;
        //busco la proposal
        let (current_proposal, nmp) = proposal::table
            .inner_join(
                new_member_proposal::table.on(new_member_proposal::proposal_id.eq(proposal::id)),
            )
            .filter(proposal::id.eq(new_member_proposal_id))
            .first::<(Proposal, NewMemberProposal)>(&mut conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => AppError::NotFound,
                _ => AppError::NotFound,
            })?;

        //veo que el destination sea el mismo
        if nmp.new_member_id != destination {
            return Err(AppError::Unauthorized);
        }
        //veo que este approved
        if current_proposal.status != MyProposalStatus::Approved {
            return Err(AppError::Forbidden);
        }

        //elijo nuevo status
        let next_status = if approve {
            MyProposalStatus::Executed
        } else {
            MyProposalStatus::Rejected
        };
        //transaction
        let updated_proposal = conn
            .transaction::<Proposal, diesel::result::Error, _>(|this_conn| {
                //update el status
                let updated =
                    diesel::update(proposal::table.filter(proposal::id.eq(new_member_proposal_id)))
                        .set(proposal::status.eq(&next_status))
                        .get_result::<Proposal>(this_conn)?;
                //si era un true agrego al tipo en el grupo
                if approve {
                    let new_user_in_group = NewUserInGroup {
                        user_id: destination,
                        group_id: updated.group_id,
                        role: Some(MyGroupRole::Member),
                    };

                    diesel::insert_into(user_in_group::table)
                        .values(&new_user_in_group)
                        .returning(UserInGroup::as_returning())
                        .get_result::<UserInGroup>(this_conn)?;
                }

                Ok(updated)
            })
            .map_err(|err| match err {
                diesel::result::Error::NotFound => AppError::NotFound,
                _ => AppError::NotFound,
            })?;

        Ok(NewMemberProposalExpanded {
            proposal: updated_proposal,
            new_member_proposal: nmp,
            proposal_type: ProposalType::NewMember,
        })
    }
    fn find_new_member_received_by(
        &self,
        destination: Uuid,
    ) -> Result<Vec<ReceivedNewMemberProposalExpanded>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = new_member_proposal::table
            .inner_join(proposal::table.on(new_member_proposal::proposal_id.eq(proposal::id)))
            .inner_join(user::table.on(proposal::created_by.eq(user::id)))
            .inner_join(group::table.on(proposal::group_id.eq(group::id)))
            .filter(new_member_proposal::new_member_id.eq(destination))
            .filter(proposal::status.eq(MyProposalStatus::Approved))
            .select((
                new_member_proposal::all_columns,
                proposal::all_columns,
                user::name,
                group::name,
            ))
            .load::<(NewMemberProposal, Proposal, String, String)>(&mut conn)?;

        let parsed: Vec<ReceivedNewMemberProposalExpanded> = result
            .into_iter()
            .map(
                |(nmp, p, sender_name, group_name)| ReceivedNewMemberProposalExpanded {
                    sender_name,
                    group_name,
                    proposal: p,
                    new_member_proposal: nmp,
                    proposal_type: ProposalType::NewMember,
                },
            )
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
            let mut proposal = diesel::insert_into(proposal::table)
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

            // TODO: Handle voting system
            // For now, update to Approved.
            // === REMOVE LATER ===
            proposal = diesel::update(proposal::table.filter(proposal::id.eq(proposal.id)))
                .set(ProposalUpdate {
                    status: MyProposalStatus::Approved,
                })
                .get_result(this_conn)?;
            // === REMOVE LATER ===

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
