#[path = "application_html_actions_auth/admin_html_auth_action_route_registry.rs"]
mod admin_html_auth_action_route_registry;
#[path = "application_html_actions_auth/change_password.rs"]
mod change_password;
#[path = "application_html_actions_auth/sign_in.rs"]
mod sign_in;
#[path = "application_html_actions_auth/sign_out.rs"]
mod sign_out;

pub(super) use admin_html_auth_action_route_registry::router;
