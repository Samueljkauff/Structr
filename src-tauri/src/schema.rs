// @generated automatically by Diesel CLI.

diesel::table! {
    folder_metadata (path) {
        path -> Text,
        description -> Text,
        updated_at -> Timestamp,
    }
}
