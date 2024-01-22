-- Your SQL goes here

CREATE TABLE posts
(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    author_id   INTEGER      NOT NULL,
    completed   BOOLEAN      NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);