CREATE TABLE files (
    id BIGINT PRIMARY KEY,
    display_name VARCHAR(255),
    owner BIGINT REFERENCES users (id)
)
