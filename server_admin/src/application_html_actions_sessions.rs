#[path = "application_html_actions_sessions/revoke_session.rs"]
mod revoke_session;
#[path = "application_html_actions_sessions/router.rs"]
mod router;

pub(super) use revoke_session::AdminHtmlSessionActionRouteRegistry;
pub(super) use router::router;
