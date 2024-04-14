// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        role -> Nullable<Integer>,
        created_at -> Timestamp,
    }
}
