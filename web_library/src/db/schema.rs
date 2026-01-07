// @generated automatically by Diesel CLI.

diesel::table! {
    search_history (id) {
        id -> Int4,
        user_id -> Int4,
        query_text -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        password_hash -> Text,
    }
}

diesel::joinable!(search_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(search_history, users,);
