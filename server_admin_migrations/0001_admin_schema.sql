CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    must_change_password BOOLEAN NOT NULL DEFAULT TRUE,
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
CREATE TYPE permission_action_key AS ENUM ('create', 'read', 'update', 'delete');
CREATE TABLE permission_actions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    key permission_action_key NOT NULL UNIQUE
);
CREATE VIEW permission_actions_read AS
SELECT id, key::TEXT AS key
FROM permission_actions;
CREATE TYPE permission_resource_key AS ENUM (
    'users',
    'roles',
    'role_rules',
    'user_roles',
    'access_sessions',
    'system_settings',
    'permission_actions',
    'permission_resources',
    'permission_resource_actions',
    'audit_log',
    'cleanup_status',
    'login_attempts',
    'rate_limits',
    'refresh_tokens',
    'rules',
    'metrics',
    'openapi',
    'tables',
    'basemaps',
    'layer_groups',
    'layers',
    'project_groups',
    'projects',
    'properties',
    'features',
    'value_items'
);
CREATE TABLE permission_resources (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    key permission_resource_key NOT NULL UNIQUE
);
CREATE VIEW permission_resources_read AS
SELECT id, key::TEXT AS key
FROM permission_resources;
CREATE TABLE permission_resource_actions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    permission_resource_id BIGINT NOT NULL REFERENCES permission_resources(id) ON DELETE CASCADE,
    permission_action_id BIGINT NOT NULL REFERENCES permission_actions(id) ON DELETE CASCADE,
    UNIQUE (permission_resource_id, permission_action_id)
);
CREATE INDEX permission_resource_actions_permission_action_id_idx
    ON permission_resource_actions (permission_action_id);
CREATE TABLE rules (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    permission_resource_action_id BIGINT NOT NULL REFERENCES permission_resource_actions(id) ON DELETE CASCADE,
    basemap_id BIGINT,
    layer_group_id BIGINT,
    layer_id BIGINT,
    project_group_id BIGINT,
    project_id BIGINT,
    property_id BIGINT,
    role_id BIGINT,
    user_id BIGINT,
    feature_id BIGINT,
    value_item_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT rules_at_most_one_scope CHECK (
        num_nonnulls(
            basemap_id,
            layer_group_id,
            layer_id,
            project_group_id,
            project_id,
            property_id,
            role_id,
            user_id,
            feature_id,
            value_item_id
        ) <= 1
    ),
    UNIQUE NULLS NOT DISTINCT (
        permission_resource_action_id,
        basemap_id,
        layer_group_id,
        layer_id,
        project_group_id,
        project_id,
        property_id,
        role_id,
        user_id,
        feature_id,
        value_item_id
    )
);
CREATE INDEX rules_permission_resource_action_id_idx
    ON rules (permission_resource_action_id);
CREATE VIEW rules_read AS
SELECT
    rules.id,
    rules.permission_resource_action_id,
    rules.basemap_id,
    rules.layer_group_id,
    rules.layer_id,
    rules.project_group_id,
    rules.project_id,
    rules.property_id,
    rules.role_id,
    rules.user_id,
    rules.feature_id,
    rules.value_item_id,
    rules.created_at,
    rules.updated_at,
    permission_resources.key::TEXT || ':' || permission_actions.key::TEXT AS name,
    num_nonnulls(
        rules.basemap_id,
        rules.layer_group_id,
        rules.layer_id,
        rules.project_group_id,
        rules.project_id,
        rules.property_id,
        rules.role_id,
        rules.user_id,
        rules.feature_id,
        rules.value_item_id
    ) = 0 AS is_global
FROM rules
JOIN permission_resource_actions
    ON permission_resource_actions.id = rules.permission_resource_action_id
JOIN permission_resources
    ON permission_resources.id = permission_resource_actions.permission_resource_id
JOIN permission_actions
    ON permission_actions.id = permission_resource_actions.permission_action_id;
CREATE VIEW auth_rules AS
SELECT id, name
FROM rules_read
WHERE is_global;
CREATE TABLE user_roles (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, role_id)
);
CREATE INDEX user_roles_role_id_idx ON user_roles (role_id);
CREATE TABLE role_rules (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    role_id BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    rule_id BIGINT NOT NULL REFERENCES rules(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (role_id, rule_id)
);
CREATE INDEX role_rules_rule_id_idx ON role_rules (rule_id);
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
    tab_title TEXT NOT NULL DEFAULT 'Admin',
    main_logo TEXT NOT NULL DEFAULT 'https://example.com/admin-logo.svg',
    primary_color TEXT NOT NULL DEFAULT '#5b55e7',
    default_admin_route TEXT NOT NULL DEFAULT '/admin/users',
    organization_name TEXT NOT NULL DEFAULT 'Admin',
    organization_contacts TEXT NOT NULL DEFAULT 'support@example.com',
    support_url TEXT NOT NULL DEFAULT 'https://example.com/support',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT system_settings_singleton CHECK (id = 1),
    CONSTRAINT system_settings_site_name_not_empty CHECK (char_length(btrim(site_name)) > 0),
    CONSTRAINT system_settings_default_route_format CHECK (default_admin_route LIKE '/admin%')
);
INSERT INTO system_settings (id) VALUES (1);
CREATE TABLE rate_limits (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    scope TEXT NOT NULL,
    subject TEXT NOT NULL,
    window_started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    request_count BIGINT NOT NULL DEFAULT 0,
    UNIQUE (scope, subject),
    CONSTRAINT rate_limits_scope_not_empty CHECK (char_length(scope) > 0),
    CONSTRAINT rate_limits_subject_not_empty CHECK (char_length(subject) > 0),
    CONSTRAINT rate_limits_request_count_nonnegative CHECK (request_count >= 0)
);
CREATE INDEX rate_limits_window_idx ON rate_limits (window_started_at);
CREATE TABLE cleanup_status (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    singleton BOOLEAN NOT NULL UNIQUE DEFAULT TRUE CHECK (singleton),
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
