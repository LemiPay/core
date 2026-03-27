// @generated automatically by Diesel CLI.

diesel::table! {
    group (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
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
