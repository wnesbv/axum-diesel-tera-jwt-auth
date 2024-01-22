// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        author_id -> Int4,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (session_token) {
        session_token -> Bytea,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(posts -> users (author_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    sessions,
    users,
);
