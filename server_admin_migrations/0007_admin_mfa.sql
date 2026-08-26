ALTER TABLE admin_access_sessions
    ADD COLUMN IF NOT EXISTS mfa_verified_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS admin_user_mfa (
    user_id BIGINT PRIMARY KEY REFERENCES admin_users(id) ON DELETE CASCADE,
    encrypted_secret BYTEA NOT NULL,
    secret_nonce BYTEA NOT NULL,
    pending_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    enabled_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT admin_user_mfa_encrypted_secret_not_empty CHECK (octet_length(encrypted_secret) > 0),
    CONSTRAINT admin_user_mfa_nonce_length CHECK (octet_length(secret_nonce) = 12),
    CONSTRAINT admin_user_mfa_enabled_after_pending CHECK (enabled_at IS NULL OR enabled_at >= pending_created_at)
);

CREATE TABLE IF NOT EXISTS admin_mfa_recovery_codes (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE,
    code_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    used_at TIMESTAMPTZ,
    CONSTRAINT admin_mfa_recovery_code_hash_not_empty CHECK (char_length(code_hash) > 0),
    CONSTRAINT admin_mfa_recovery_code_used_after_created CHECK (used_at IS NULL OR used_at >= created_at),
    UNIQUE (user_id, code_hash)
);

CREATE INDEX IF NOT EXISTS admin_mfa_recovery_codes_unused_idx
    ON admin_mfa_recovery_codes (user_id, id)
    WHERE used_at IS NULL;
