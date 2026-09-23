ALTER TABLE permissions RENAME TO rules;
ALTER TABLE rules RENAME CONSTRAINT permissions_pkey TO rules_pkey;
ALTER TABLE rules RENAME CONSTRAINT permissions_name_key TO rules_name_key;
ALTER TABLE rules RENAME CONSTRAINT permissions_name_length TO rules_name_length;
ALTER TABLE rules RENAME CONSTRAINT permissions_name_format TO rules_name_format;
ALTER TABLE role_permissions RENAME TO role_rules;
ALTER TABLE role_rules RENAME COLUMN permission_id TO rule_id;
ALTER TABLE role_rules RENAME CONSTRAINT role_permissions_pkey TO role_rules_pkey;
ALTER TABLE role_rules RENAME CONSTRAINT role_permissions_role_id_fkey TO role_rules_role_id_fkey;
ALTER TABLE role_rules RENAME CONSTRAINT role_permissions_permission_id_fkey TO role_rules_rule_id_fkey;
ALTER TABLE role_rules RENAME CONSTRAINT role_permissions_role_id_permission_id_key TO role_rules_role_id_rule_id_key;
ALTER INDEX role_permissions_permission_id_idx RENAME TO role_rules_rule_id_idx;
UPDATE rules
SET name = CASE name
    WHEN 'permissions:read' THEN 'rules:read'
    WHEN 'role_permissions:create' THEN 'role_rules:create'
    WHEN 'role_permissions:delete' THEN 'role_rules:delete'
    WHEN 'role_permissions:read' THEN 'role_rules:read'
    WHEN 'role_permissions:update' THEN 'role_rules:update'
    ELSE name
END
WHERE name IN (
    'permissions:read',
    'role_permissions:create',
    'role_permissions:delete',
    'role_permissions:read',
    'role_permissions:update'
);
