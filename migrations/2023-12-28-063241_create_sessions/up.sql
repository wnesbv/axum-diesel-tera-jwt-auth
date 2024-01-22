-- Your SQL goes here

CREATE TABLE sessions (
    session_token BYTEA PRIMARY KEY,
    user_id integer REFERENCES users (id) ON DELETE CASCADE
);