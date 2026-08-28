pub(crate) use admin_html_auth_action_route_registry::admin_html_auth_action_router;

// Root-owned module compatibility wrappers.
mod admin_html_auth_action_route_registry {
    pub use crate::admin_html_auth_action_route_registry::*;
}
mod change_password {
    pub use crate::change_password::*;
}
mod sign_in {
    pub use crate::sign_in::*;
}
mod sign_out {
    pub use crate::sign_out::*;
}
