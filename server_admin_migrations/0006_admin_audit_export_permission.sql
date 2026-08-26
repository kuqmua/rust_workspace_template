INSERT INTO admin_permissions (name) VALUES ('audit_log:export')
ON CONFLICT (name) DO NOTHING;

INSERT INTO admin_role_permissions (role_id, permission_id)
SELECT admin_roles.id, admin_permissions.id
FROM admin_roles
CROSS JOIN admin_permissions
WHERE admin_roles.name = 'admin'
  AND admin_permissions.name = 'audit_log:export'
ON CONFLICT (role_id, permission_id) DO NOTHING;
