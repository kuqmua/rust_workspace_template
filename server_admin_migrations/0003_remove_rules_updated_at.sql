DROP VIEW auth_rules;
DROP VIEW rules_read;
ALTER TABLE rules DROP COLUMN updated_at;
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
