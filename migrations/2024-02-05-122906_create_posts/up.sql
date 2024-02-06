-- Your SQL goes here
-- set default value of img column to '/img/post/post.jpg'
CREATE TABLE posts (
    id          SERIAL PRIMARY KEY,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    img         TEXT        NOT NULL DEFAULT '/img/post/post.jpg',
    author_id   INTEGER     NOT NULL,
    completed   BOOLEAN     NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);