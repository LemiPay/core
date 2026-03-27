// @generated automatically by Diesel CLI.

pub mod sql_types {
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

diesel::allow_tables_to_appear_in_same_query!(group, user,);
