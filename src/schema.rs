// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        due_date -> Nullable<Timestamp>,
        status -> Bool,
    }
}
