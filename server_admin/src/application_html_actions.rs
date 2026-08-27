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

#[path = "application_html_actions/assignment_action.rs"]
mod assignment_action;
#[path = "application_html_actions/root.rs"]
mod root;
#[path = "application_html_actions/router.rs"]
mod router;

use assignment_action::assignment_action;
pub(super) use root::AdminHtmlActionRouteRegistry;
#[cfg(test)]
pub(super) use root::root;
pub(super) use router::router;
