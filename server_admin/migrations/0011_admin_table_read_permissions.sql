ALTER FUNCTION admin_set_updated_at() RENAME TO set_updated_at;
ALTER FUNCTION admin_audit_log_append_only() RENAME TO audit_log_append_only;

ALTER TRIGGER admin_users_set_updated_at ON users RENAME TO users_set_updated_at;
ALTER TRIGGER admin_roles_set_updated_at ON roles RENAME TO roles_set_updated_at;
ALTER TRIGGER admin_system_settings_set_updated_at ON system_settings RENAME TO system_settings_set_updated_at;
ALTER TRIGGER admin_audit_log_append_only_guard ON audit_log RENAME TO audit_log_append_only_guard;

CREATE OR REPLACE FUNCTION audit_log_append_only() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' AND current_setting('app.admin_audit_cleanup', TRUE) = 'on' THEN
        RETURN OLD;
    END IF;
    RAISE EXCEPTION 'audit_log is append-only';
END;
$$;

DO $$
DECLARE
    item RECORD;
BEGIN
    FOR item IN
        SELECT constraint_value.conrelid::regclass AS table_name, constraint_value.conname AS object_name
        FROM pg_constraint constraint_value
        JOIN pg_namespace namespace ON namespace.oid = constraint_value.connamespace
        WHERE namespace.nspname = current_schema()
          AND constraint_value.conname LIKE 'admin\_%' ESCAPE '\'
    LOOP
        EXECUTE format(
            'ALTER TABLE %s RENAME CONSTRAINT %I TO %I',
            item.table_name,
            item.object_name,
            substring(item.object_name FROM 7)
        );
    END LOOP;

    FOR item IN
        SELECT indexname AS object_name
        FROM pg_indexes
        WHERE schemaname = current_schema()
          AND indexname LIKE 'admin\_%' ESCAPE '\'
    LOOP
        EXECUTE format(
            'ALTER INDEX %I RENAME TO %I',
            item.object_name,
            substring(item.object_name FROM 7)
        );
    END LOOP;
END;
$$;

INSERT INTO permissions (name) VALUES
    ('access_sessions:read'),
    ('cleanup_status:read'),
    ('login_attempts:read'),
    ('mfa_recovery_codes:read'),
    ('rate_limits:read'),
    ('refresh_tokens:read'),
    ('tables:read'),
    ('user_mfa:read')
ON CONFLICT (name) DO NOTHING;

INSERT INTO role_permissions (role_id, permission_id)
SELECT roles.id, permissions.id
FROM roles
CROSS JOIN permissions
WHERE roles.name = 'admin'
  AND permissions.name IN (
      'access_sessions:read',
      'cleanup_status:read',
      'login_attempts:read',
      'mfa_recovery_codes:read',
      'rate_limits:read',
      'refresh_tokens:read',
      'tables:read',
      'user_mfa:read'
  )
ON CONFLICT (role_id, permission_id) DO NOTHING;
