-- Your SQL goes here
-- set default value of img column to '/img/user/user.jpg'
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    img TEXT NOT NULL DEFAULT '/img/user/user.jpg',
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);