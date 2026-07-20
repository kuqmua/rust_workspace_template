CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT users_login_length CHECK (char_length(login) BETWEEN 3 AND 128),
    CONSTRAINT users_login_format CHECK (login = lower(login) AND login ~ '^[a-z0-9_.-]+$'),
    CONSTRAINT users_display_name_length CHECK (char_length(display_name) BETWEEN 1 AND 256),
    CONSTRAINT users_display_name_trimmed CHECK (display_name = btrim(display_name)),
    CONSTRAINT users_password_hash_not_empty CHECK (char_length(password_hash) > 0)
);
CREATE UNIQUE INDEX users_login_lower_unq ON users ((lower(login)));

CREATE TABLE roles (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT roles_name_length CHECK (char_length(name) BETWEEN 1 AND 128),
    CONSTRAINT roles_name_format CHECK (name = lower(name) AND name ~ '^[a-z0-9_.-]+$')
);

CREATE TABLE permissions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT permissions_name_length CHECK (char_length(name) BETWEEN 3 AND 128),
    CONSTRAINT permissions_name_format CHECK (name = lower(name) AND name ~ '^[a-z0-9_]+:[a-z0-9_]+$')
);

CREATE TABLE user_roles (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, role_id)
);
CREATE INDEX user_roles_role_id_idx ON user_roles (role_id);

CREATE TABLE role_permissions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (role_id, permission_id)
);
CREATE INDEX role_permissions_permission_id_idx ON role_permissions (permission_id);

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT refresh_tokens_hash_not_empty CHECK (char_length(token_hash) > 0),
    CONSTRAINT refresh_tokens_expires_after_created CHECK (expires_at > created_at),
    CONSTRAINT refresh_tokens_revoked_after_created CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);
CREATE INDEX refresh_tokens_user_expiry_idx ON refresh_tokens (user_id, expires_at);

CREATE TABLE access_sessions (
    id UUID PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_identifier_hash TEXT NOT NULL UNIQUE,
    csrf_token_hash TEXT NOT NULL,
    token_context_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    mfa_verified_at TIMESTAMPTZ,
    CONSTRAINT access_sessions_token_hash_not_empty CHECK (char_length(token_identifier_hash) > 0),
    CONSTRAINT access_sessions_csrf_hash_not_empty CHECK (char_length(csrf_token_hash) > 0),
    CONSTRAINT access_sessions_context_hash_format CHECK (token_context_hash ~ '^[0-9a-f]{64}$'),
    CONSTRAINT access_sessions_expires_after_created CHECK (expires_at > created_at),
    CONSTRAINT access_sessions_revoked_after_created CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);
CREATE INDEX access_sessions_user_expiry_idx ON access_sessions (user_id, expires_at);
CREATE INDEX access_sessions_context_active_idx
    ON access_sessions (user_id, token_context_hash, expires_at)
    WHERE revoked_at IS NULL;

CREATE TABLE login_attempts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL,
    ip_address INET,
    succeeded BOOLEAN NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT login_attempts_login_not_empty CHECK (char_length(login) > 0)
);
CREATE INDEX login_attempts_login_ip_time_idx ON login_attempts (login, ip_address, attempted_at DESC);

CREATE TABLE audit_log (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT,
    user_login TEXT,
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    resource_id TEXT,
    request_id UUID,
    succeeded BOOLEAN NOT NULL,
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT audit_log_action_format CHECK (action = lower(action) AND action ~ '^[a-z0-9_]+$'),
    CONSTRAINT audit_log_resource_format CHECK (resource = lower(resource) AND resource ~ '^[a-z0-9_]+$'),
    CONSTRAINT audit_log_user_login_not_empty CHECK (user_login IS NULL OR char_length(user_login) > 0)
);
CREATE INDEX audit_log_user_time_idx ON audit_log (user_id, created_at DESC);
CREATE INDEX audit_log_resource_time_idx ON audit_log (resource, resource_id, created_at DESC);

CREATE TABLE system_settings (
    id SMALLINT PRIMARY KEY DEFAULT 1,
    site_name TEXT NOT NULL DEFAULT 'Admin',
    tab_title TEXT,
    main_logo TEXT,
    primary_color TEXT,
    default_admin_route TEXT NOT NULL DEFAULT '/admin/users',
    organization_name TEXT,
    organization_contacts TEXT,
    support_url TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT system_settings_singleton CHECK (id = 1),
    CONSTRAINT system_settings_site_name_not_empty CHECK (char_length(btrim(site_name)) > 0),
    CONSTRAINT system_settings_default_route_format CHECK (default_admin_route LIKE '/admin%')
);
INSERT INTO system_settings (id) VALUES (1);

CREATE TABLE rate_limits (
    scope TEXT NOT NULL,
    subject TEXT NOT NULL,
    window_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    request_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (scope, subject),
    CONSTRAINT rate_limits_scope_not_empty CHECK (char_length(scope) > 0),
    CONSTRAINT rate_limits_subject_not_empty CHECK (char_length(subject) > 0),
    CONSTRAINT rate_limits_request_count_nonnegative CHECK (request_count >= 0)
);
CREATE INDEX rate_limits_window_idx ON rate_limits (window_started_at);

CREATE TABLE user_mfa (
    user_id BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    encrypted_secret BYTEA NOT NULL,
    secret_nonce BYTEA NOT NULL,
    pending_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    enabled_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_totp_counter BIGINT,
    CONSTRAINT user_mfa_encrypted_secret_not_empty CHECK (octet_length(encrypted_secret) > 0),
    CONSTRAINT user_mfa_nonce_length CHECK (octet_length(secret_nonce) = 12),
    CONSTRAINT user_mfa_enabled_after_pending CHECK (enabled_at IS NULL OR enabled_at >= pending_created_at),
    CONSTRAINT user_mfa_last_totp_counter_nonnegative CHECK (last_totp_counter IS NULL OR last_totp_counter >= 0)
);

CREATE TABLE mfa_recovery_codes (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    used_at TIMESTAMPTZ,
    CONSTRAINT mfa_recovery_code_hash_not_empty CHECK (char_length(code_hash) > 0),
    CONSTRAINT mfa_recovery_code_used_after_created CHECK (used_at IS NULL OR used_at >= created_at),
    UNIQUE (user_id, code_hash)
);
CREATE INDEX mfa_recovery_codes_unused_idx
    ON mfa_recovery_codes (user_id, id)
    WHERE used_at IS NULL;

CREATE TABLE cleanup_status (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    last_success_at TIMESTAMPTZ NOT NULL,
    last_deleted_rows BIGINT NOT NULL CHECK (last_deleted_rows >= 0)
);

CREATE FUNCTION set_updated_at() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;
CREATE TRIGGER users_set_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER roles_set_updated_at BEFORE UPDATE ON roles FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER system_settings_set_updated_at BEFORE UPDATE ON system_settings FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE FUNCTION audit_log_append_only() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' AND current_setting('app.admin_audit_cleanup', TRUE) = 'on' THEN
        RETURN OLD;
    END IF;
    RAISE EXCEPTION 'audit_log is append-only';
END;
$$;
CREATE TRIGGER audit_log_append_only_guard BEFORE UPDATE OR DELETE ON audit_log FOR EACH ROW EXECUTE FUNCTION audit_log_append_only();

INSERT INTO permissions (name) VALUES
    ('access_sessions:read'),
    ('audit_log:export'),
    ('audit_log:read'),
    ('cleanup_status:read'),
    ('login_attempts:read'),
    ('metrics:read'),
    ('mfa_recovery_codes:read'),
    ('openapi:read'),
    ('permissions:read'),
    ('rate_limits:read'),
    ('refresh_tokens:read'),
    ('role_permissions:create'),
    ('role_permissions:delete'),
    ('role_permissions:read'),
    ('role_permissions:update'),
    ('roles:create'),
    ('roles:delete'),
    ('roles:read'),
    ('roles:update'),
    ('system_settings:read'),
    ('system_settings:update'),
    ('tables:read'),
    ('user_mfa:read'),
    ('user_roles:create'),
    ('user_roles:delete'),
    ('user_roles:read'),
    ('user_roles:update'),
    ('users:create'),
    ('users:delete'),
    ('users:read'),
    ('users:update');

INSERT INTO roles (name, is_system) VALUES ('admin', TRUE);
INSERT INTO role_permissions (role_id, permission_id)
SELECT roles.id, permissions.id
FROM roles
CROSS JOIN permissions
WHERE roles.name = 'admin';
