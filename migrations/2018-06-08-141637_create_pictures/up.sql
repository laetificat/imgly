CREATE TABLE pictures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NULL,
    score INTEGER NOT NULL DEFAULT 0,
    file TEXT NOT NULL,
    album_id TEXT REFERENCES albums (id),
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_date TIMESTAMP NULL
)