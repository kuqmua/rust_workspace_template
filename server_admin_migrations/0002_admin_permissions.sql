INSERT INTO admin_permissions (name) VALUES
    ('audit_log:read'),
    ('metrics:read'),
    ('openapi:read'),
    ('permissions:read'),
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
    ('user_roles:create'),
    ('user_roles:delete'),
    ('user_roles:read'),
    ('user_roles:update'),
    ('users:create'),
    ('users:delete'),
    ('users:read'),
    ('users:update')
ON CONFLICT (name) DO NOTHING;
INSERT INTO admin_roles (name, is_system) VALUES ('admin', TRUE)
ON CONFLICT (name) DO UPDATE SET is_system = TRUE;
INSERT INTO admin_role_permissions (role_id, permission_id)
SELECT admin_roles.id, admin_permissions.id
FROM admin_roles
CROSS JOIN admin_permissions
WHERE admin_roles.name = 'admin'
ON CONFLICT (role_id, permission_id) DO NOTHING;
