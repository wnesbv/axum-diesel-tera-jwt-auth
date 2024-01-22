-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    username text NOT NULL UNIQUE, -- CHECK (name <> '')
    password text NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);