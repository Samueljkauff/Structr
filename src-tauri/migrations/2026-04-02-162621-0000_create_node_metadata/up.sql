-- Your SQL goes here

CREATE TABLE folder_metadata (
    path TEXT NOT NULL PRIMARY KEY,
    description TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL
)