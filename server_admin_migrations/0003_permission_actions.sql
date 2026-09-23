CREATE TABLE permission_actions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    CONSTRAINT permission_actions_key_value CHECK (key IN ('create', 'read', 'update', 'delete'))
);
INSERT INTO permission_actions (id, key) OVERRIDING SYSTEM VALUE VALUES
    (1, 'create'),
    (2, 'read'),
    (3, 'update'),
    (4, 'delete');
INSERT INTO rules (name) VALUES ('permission_actions:read')
ON CONFLICT (name) DO NOTHING;
INSERT INTO role_rules (role_id, rule_id)
SELECT roles.id, rules.id
FROM roles
CROSS JOIN rules
WHERE roles.name = 'admin' AND rules.name = 'permission_actions:read'
ON CONFLICT (role_id, rule_id) DO NOTHING;
