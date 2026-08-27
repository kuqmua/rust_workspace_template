#[path = "application_html_actions_roles/admin_html_role_action_route_registry.rs"]
mod admin_html_role_action_route_registry;
#[path = "application_html_actions_roles/create_role.rs"]
mod create_role;
#[path = "application_html_actions_roles/delete_role.rs"]
mod delete_role;
#[path = "application_html_actions_roles/role_permissions.rs"]
mod role_permissions;
#[path = "application_html_actions_roles/update_role.rs"]
mod update_role;

pub(super) use admin_html_role_action_route_registry::router;
