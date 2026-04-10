// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_member_status"))]
    pub struct GroupMemberStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_role"))]
    pub struct GroupRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_status"))]
    pub struct GroupStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "proposal_status"))]
    pub struct ProposalStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vote_type"))]
    pub struct VoteType;
}

diesel::table! {
    currency (currency_id) {
        currency_id -> Uuid,
        name -> Text,
        ticker -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupStatus;

    group (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        status -> GroupStatus,
        created_at -> Date,
        updated_at -> Date,
    }
}

diesel::table! {
    new_member_proposal (proposal_id) {
        proposal_id -> Uuid,
        new_member_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProposalStatus;

    proposal (id) {
        id -> Uuid,
        group_id -> Uuid,
        created_by -> Uuid,
        status -> ProposalStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupRole;
    use super::sql_types::GroupMemberStatus;

    user_in_group (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
        role -> GroupRole,
        status -> GroupMemberStatus,
        joined_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_wallet (id) {
        id -> Uuid,
        address -> Text,
        user_id -> Uuid,
        currency_id -> Uuid,
        balance -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VoteType;

    vote (proposal_id, user_id) {
        proposal_id -> Uuid,
        user_id -> Uuid,
        value -> VoteType,
        created_at -> Timestamp,
    }
}

diesel::joinable!(new_member_proposal -> proposal (proposal_id));
diesel::joinable!(new_member_proposal -> user (new_member_id));
diesel::joinable!(proposal -> group (group_id));
diesel::joinable!(proposal -> user (created_by));
diesel::joinable!(user_in_group -> group (group_id));
diesel::joinable!(user_in_group -> user (user_id));
diesel::joinable!(vote -> proposal (proposal_id));
diesel::joinable!(vote -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    currency,
    group,
    new_member_proposal,
    proposal,
    user,
    user_in_group,
    user_wallet,
    vote,
);
