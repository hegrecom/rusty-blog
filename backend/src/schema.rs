// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
