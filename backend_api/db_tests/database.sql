CREATE DATABASE testing_database;
\c testing_database;

DROP TABLE IF EXISTS guild_file;
DROP TABLE IF EXISTS guild;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS state;

CREATE TABLE IF NOT EXISTS state (
    csrf_token VARCHAR(30) PRIMARY KEY,
    pkce_verifier VARCHAR(50) NOT NULL,
    expires timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '30 minutes'
);

CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    avatar VARCHAR(50),
    time_added timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS files (
    id BIGINT PRIMARY KEY,
    display_name VARCHAR(50),
    owner BIGINT REFERENCES users (id),
    is_deleted bool DEFAULT FALSE,
    is_public bool DEFAULT FALSE,
    time_added timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS guild (
    id BIGINT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    icon VARCHAR(50),
    icon_hash VARCHAR(50),
    time_added timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS guild_file(
    guild_id BIGINT REFERENCES guild(id),
    file_id BIGINT REFERENCES files(id),
    PRIMARY KEY (guild_id, file_id),
    is_deleted bool DEFAULT FALSE,
    time_added timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);
