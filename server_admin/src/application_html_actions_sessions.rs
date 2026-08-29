pub(crate) use revoke_session::AdminHtmlSessionActionRouteRegistry;

// Root-owned module compatibility wrappers.
mod revoke_session {
    pub use super::super::revoke_session::*;
}
