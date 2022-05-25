ALTER TABLE files
ADD is_deleted bool DEFAULT FALSE;

ALTER TABLE files
ADD is_public bool DEFAULT FALSE;

ALTER TABLE files
ALTER COLUMN display_name TYPE VARCHAR(50);
