CREATE TABLE users_guilds (
    id_user BIGINT REFERENCES users(id),
    id_guild BIGINT REFERENCES guild(id),
    PRIMARY KEY (id_user, id_guild)
)
