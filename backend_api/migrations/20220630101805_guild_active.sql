ALTER TABLE guild
DROP COLUMN icon;

ALTER TABLE guild
DROP COLUMN icon_hash;

ALTER TABLE guild
ADD active bool DEFAULT true;
