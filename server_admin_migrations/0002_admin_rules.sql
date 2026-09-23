INSERT INTO rules (name) VALUES
    ('access_sessions:delete'),
    ('access_sessions:read'),
    ('audit_log:read'),
    ('cleanup_status:read'),
    ('login_attempts:read'),
    ('metrics:read'),
    ('openapi:read'),
    ('rules:read'),
    ('rate_limits:read'),
    ('refresh_tokens:read'),
    ('role_rules:create'),
    ('role_rules:delete'),
    ('role_rules:read'),
    ('role_rules:update'),
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
INSERT INTO role_rules (role_id, rule_id)
SELECT roles.id, rules.id
FROM roles
CROSS JOIN rules
WHERE roles.name = 'admin'
ON CONFLICT (role_id, rule_id) DO NOTHING;
