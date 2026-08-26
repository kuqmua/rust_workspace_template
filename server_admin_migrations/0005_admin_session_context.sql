ALTER TABLE admin_access_sessions
    ADD COLUMN token_context_hash TEXT;

UPDATE admin_access_sessions
SET token_context_hash = repeat('0', 64),
    revoked_at = COALESCE(revoked_at, NOW());

ALTER TABLE admin_access_sessions
    ALTER COLUMN token_context_hash SET NOT NULL,
    ADD CONSTRAINT admin_access_sessions_context_hash_format
        CHECK (token_context_hash ~ '^[0-9a-f]{64}$');

UPDATE admin_refresh_tokens
SET revoked_at = COALESCE(revoked_at, NOW());

CREATE INDEX admin_access_sessions_context_active_idx
    ON admin_access_sessions (user_id, token_context_hash, expires_at)
    WHERE revoked_at IS NULL;
