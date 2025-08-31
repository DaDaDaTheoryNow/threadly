// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "session_status"))]
    pub struct SessionStatus;
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        session_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        round -> Int4,
        turn_order -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    players (session_id, user_id) {
        session_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Timestamp,
        is_ready -> Bool,
        is_host -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SessionStatus;

    sessions (id) {
        id -> Uuid,
        theme -> Text,
        status -> SessionStatus,
        current_user_id_turn -> Nullable<Uuid>,
        max_rounds -> Int4,
        current_round -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    stories (id) {
        id -> Uuid,
        session_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(messages -> sessions (session_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(players -> sessions (session_id));
diesel::joinable!(players -> users (user_id));
diesel::joinable!(stories -> sessions (session_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    players,
    sessions,
    stories,
    users,
);
