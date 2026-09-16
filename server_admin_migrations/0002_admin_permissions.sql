INSERT INTO permissions (name) VALUES
    ('access_sessions:read'),
    ('audit_log:read'),
    ('cleanup_status:read'),
    ('login_attempts:read'),
    ('metrics:read'),
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
    ('user_roles:create'),
    ('user_roles:delete'),
    ('user_roles:read'),
    ('user_roles:update'),
    ('users:create'),
    ('users:delete'),
    ('users:read'),
    ('users:update')
ON CONFLICT (name) DO NOTHING;
INSERT INTO roles (name, is_system) VALUES ('admin', TRUE)
ON CONFLICT (name) DO UPDATE SET is_system = TRUE;
INSERT INTO role_permissions (role_id, permission_id)
SELECT roles.id, permissions.id
FROM roles
CROSS JOIN permissions
WHERE roles.name = 'admin'
ON CONFLICT (role_id, permission_id) DO NOTHING;
