// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        password -> Text,
        auth_token -> Nullable<Text>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
