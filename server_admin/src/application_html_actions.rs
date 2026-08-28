#[path = "application_html_actions_auth.rs"]
mod auth;
#[path = "application_html_actions_roles.rs"]
mod roles;
#[path = "application_html_actions_sessions.rs"]
mod sessions;
#[path = "application_html_actions_settings.rs"]
mod settings;
#[path = "application_html_actions_users.rs"]
mod users;

#[path = "admin_html_action_router.rs"]
mod admin_html_action_router;
#[path = "assignment_action.rs"]
mod assignment_action;
#[path = "root.rs"]
mod root;

pub(super) use admin_html_action_router::admin_html_action_router;
use assignment_action::assignment_action;
pub(super) use root::AdminHtmlActionRouteRegistry;
#[cfg(test)]
pub(super) use root::root;
