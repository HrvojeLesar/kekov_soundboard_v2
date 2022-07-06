-- WARN: This requires the database to have pre-enabled pg_trgm extension or the user be a superuser
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX display_name_gist_index ON files USING GIST(display_name gist_trgm_ops);
