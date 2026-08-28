pub(crate) use admin_html_session_action_router::admin_html_session_action_router;
pub(crate) use revoke_session::AdminHtmlSessionActionRouteRegistry;

// Root-owned module compatibility wrappers.
mod admin_html_session_action_router {
    pub use crate::admin_html_session_action_router::*;
}
mod revoke_session {
    pub use crate::revoke_session::*;
}
