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

diesel::joinable!(user_in_group -> group (group_id));
diesel::joinable!(user_in_group -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(group, user, user_in_group,);
