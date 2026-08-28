#[path = "admin_html_auth_action_route_registry.rs"]
mod admin_html_auth_action_route_registry;
#[path = "change_password.rs"]
mod change_password;
#[path = "sign_in.rs"]
mod sign_in;
#[path = "sign_out.rs"]
mod sign_out;

pub(super) use admin_html_auth_action_route_registry::admin_html_auth_action_router;
