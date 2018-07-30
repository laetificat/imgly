CREATE TABLE albums (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NULL,
    cover_image TEXT NULL,
    username TEXT REFERENCES users (username),
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_date TIMESTAMP NULL
)