-- Add migration script here
CREATE TABLE Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(128) NOT NULL,
    password_hash VARCHAR(128) NOT NULL
)
