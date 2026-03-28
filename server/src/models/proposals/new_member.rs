use crate::schema::new_member_proposal;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = new_member_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMemberProposal {
    pub proposal_id: Uuid,
    pub new_member_id: Uuid,
}
