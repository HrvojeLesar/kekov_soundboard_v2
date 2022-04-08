CREATE TABLE guild_file(
    guild_id BIGINT REFERENCES guild(id),
    file_id BIGINT REFERENCES files(id),
    PRIMARY KEY (guild_id, file_id)
)
