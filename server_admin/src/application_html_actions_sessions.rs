#[path = "admin_html_session_action_router.rs"]
mod admin_html_session_action_router;
#[path = "revoke_session.rs"]
mod revoke_session;

pub(super) use admin_html_session_action_router::admin_html_session_action_router;
pub(super) use revoke_session::AdminHtmlSessionActionRouteRegistry;
