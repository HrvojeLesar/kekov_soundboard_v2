-- Add migration script here
ALTER TABLE state ALTER COLUMN pkce_verifier SET NOT NULL;
