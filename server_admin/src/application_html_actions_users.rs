#[path = "admin_html_user_action_route_registry.rs"]
mod admin_html_user_action_route_registry;
#[path = "admin_html_user_action_router.rs"]
mod admin_html_user_action_router;
#[path = "create_user.rs"]
mod create_user;
#[path = "delete_user.rs"]
mod delete_user;
#[path = "update_user.rs"]
mod update_user;
#[path = "user_ban.rs"]
mod user_ban;
#[path = "user_password.rs"]
mod user_password;
#[path = "user_roles.rs"]
mod user_roles;

pub(super) use admin_html_user_action_router::admin_html_user_action_router;
