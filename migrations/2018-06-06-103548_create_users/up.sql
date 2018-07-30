CREATE TABLE users (
    username TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    bio TEXT NULL,
    display_image TEXT NOT NULL,
    password TEXT NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_date TIMESTAMP NULL
);