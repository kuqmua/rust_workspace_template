#[path = "admin_html_role_action_route_registry.rs"]
mod admin_html_role_action_route_registry;
#[path = "create_role.rs"]
mod create_role;
#[path = "delete_role.rs"]
mod delete_role;
#[path = "role_permissions.rs"]
mod role_permissions;
#[path = "update_role.rs"]
mod update_role;

pub(super) use admin_html_role_action_route_registry::admin_html_role_action_router;
