// @generated automatically by Diesel CLI.

diesel::table! {
    file_moves (id) {
        id -> Integer,
        from_path -> Text,
        to_path -> Text,
        moved_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    folder_metadata (path) {
        path -> Text,
        description -> Text,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(file_moves, folder_metadata,);
