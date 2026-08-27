#[path = "application_html_actions_users/admin_html_user_action_route_registry.rs"]
mod admin_html_user_action_route_registry;
#[path = "application_html_actions_users/create_user.rs"]
mod create_user;
#[path = "application_html_actions_users/delete_user.rs"]
mod delete_user;
#[path = "application_html_actions_users/router.rs"]
mod router;
#[path = "application_html_actions_users/update_user.rs"]
mod update_user;
#[path = "application_html_actions_users/user_ban.rs"]
mod user_ban;
#[path = "application_html_actions_users/user_password.rs"]
mod user_password;
#[path = "application_html_actions_users/user_roles.rs"]
mod user_roles;

pub(super) use router::router;
