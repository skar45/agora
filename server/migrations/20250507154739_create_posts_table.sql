-- Add migration script here
CREATE TABLE body(
    id SERIAL PRIMARY KEY,
    body TEXT,
    image_uri TEXT
);

CREATE TABLE posts(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body_id INT references body(id),
    created_at timestamptz NOT NULL
);

