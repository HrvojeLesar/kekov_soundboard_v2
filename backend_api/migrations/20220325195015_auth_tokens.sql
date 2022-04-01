CREATE TABLE state (
    csrf_token VARCHAR(30) PRIMARY KEY,
    pkce_verifier VARCHAR(50),
    expires timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '30 minutes'
);
