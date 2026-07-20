CREATE TABLE admin_users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT admin_users_login_length CHECK (char_length(login) BETWEEN 3 AND 128),
    CONSTRAINT admin_users_login_format CHECK (login = lower(login) AND login ~ '^[a-z0-9_.-]+$'),
    CONSTRAINT admin_users_display_name_length CHECK (char_length(display_name) BETWEEN 1 AND 256),
    CONSTRAINT admin_users_display_name_trimmed CHECK (display_name = btrim(display_name)),
    CONSTRAINT admin_users_password_hash_not_empty CHECK (char_length(password_hash) > 0)
);
CREATE UNIQUE INDEX admin_users_login_lower_unq ON admin_users ((lower(login)));
CREATE TABLE admin_roles (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT admin_roles_name_length CHECK (char_length(name) BETWEEN 1 AND 128),
    CONSTRAINT admin_roles_name_format CHECK (name = lower(name) AND name ~ '^[a-z0-9_.-]+$')
);
CREATE TABLE admin_permissions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT admin_permissions_name_length CHECK (char_length(name) BETWEEN 3 AND 128),
    CONSTRAINT admin_permissions_name_format CHECK (name = lower(name) AND name ~ '^[a-z0-9_]+:[a-z0-9_]+$')
);
CREATE TABLE admin_user_roles (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES admin_roles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, role_id)
);
CREATE INDEX admin_user_roles_role_id_idx ON admin_user_roles (role_id);
CREATE TABLE admin_role_permissions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    role_id BIGINT NOT NULL REFERENCES admin_roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES admin_permissions(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (role_id, permission_id)
);
CREATE INDEX admin_role_permissions_permission_id_idx ON admin_role_permissions (permission_id);
CREATE TABLE admin_refresh_tokens (
    id UUID PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT admin_refresh_tokens_hash_not_empty CHECK (char_length(token_hash) > 0),
    CONSTRAINT admin_refresh_tokens_expires_after_created CHECK (expires_at > created_at),
    CONSTRAINT admin_refresh_tokens_revoked_after_created CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);
CREATE INDEX admin_refresh_tokens_user_expiry_idx ON admin_refresh_tokens (user_id, expires_at);
CREATE TABLE admin_access_sessions (
    id UUID PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES admin_users(id) ON DELETE CASCADE,
    token_identifier_hash TEXT NOT NULL UNIQUE,
    csrf_token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT admin_access_sessions_token_hash_not_empty CHECK (char_length(token_identifier_hash) > 0),
    CONSTRAINT admin_access_sessions_csrf_hash_not_empty CHECK (char_length(csrf_token_hash) > 0),
    CONSTRAINT admin_access_sessions_expires_after_created CHECK (expires_at > created_at),
    CONSTRAINT admin_access_sessions_revoked_after_created CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);
CREATE INDEX admin_access_sessions_user_expiry_idx ON admin_access_sessions (user_id, expires_at);
CREATE TABLE admin_login_attempts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL,
    ip_address INET,
    succeeded BOOLEAN NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT admin_login_attempts_login_not_empty CHECK (char_length(login) > 0)
);
CREATE INDEX admin_login_attempts_login_ip_time_idx ON admin_login_attempts (login, ip_address, attempted_at DESC);
CREATE TABLE admin_audit_log (
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
    CONSTRAINT admin_audit_log_action_format CHECK (action = lower(action) AND action ~ '^[a-z0-9_]+$'),
    CONSTRAINT admin_audit_log_resource_format CHECK (resource = lower(resource) AND resource ~ '^[a-z0-9_]+$'),
    CONSTRAINT admin_audit_log_user_login_not_empty CHECK (user_login IS NULL OR char_length(user_login) > 0)
);
CREATE INDEX admin_audit_log_user_time_idx ON admin_audit_log (user_id, created_at DESC);
CREATE INDEX admin_audit_log_resource_time_idx ON admin_audit_log (resource, resource_id, created_at DESC);
CREATE TABLE admin_system_settings (
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
    CONSTRAINT admin_system_settings_singleton CHECK (id = 1),
    CONSTRAINT admin_system_settings_site_name_not_empty CHECK (char_length(btrim(site_name)) > 0),
    CONSTRAINT admin_system_settings_default_route_format CHECK (default_admin_route LIKE '/admin%')
);
INSERT INTO admin_system_settings (id) VALUES (1);
CREATE FUNCTION admin_set_updated_at() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;
CREATE TRIGGER admin_users_set_updated_at BEFORE UPDATE ON admin_users FOR EACH ROW EXECUTE FUNCTION admin_set_updated_at();
CREATE TRIGGER admin_roles_set_updated_at BEFORE UPDATE ON admin_roles FOR EACH ROW EXECUTE FUNCTION admin_set_updated_at();
CREATE TRIGGER admin_system_settings_set_updated_at BEFORE UPDATE ON admin_system_settings FOR EACH ROW EXECUTE FUNCTION admin_set_updated_at();
CREATE FUNCTION admin_audit_log_append_only() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'admin_audit_log is append-only';
END;
$$;
CREATE TRIGGER admin_audit_log_append_only_guard BEFORE UPDATE OR DELETE ON admin_audit_log FOR EACH ROW EXECUTE FUNCTION admin_audit_log_append_only();
