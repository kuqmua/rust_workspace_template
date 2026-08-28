pub(crate) use admin_html_role_action_route_registry::admin_html_role_action_router;

// Root-owned module compatibility wrappers.
mod admin_html_role_action_route_registry {
    pub use crate::admin_html_role_action_route_registry::*;
}
mod create_role {
    pub use crate::create_role::*;
}
mod delete_role {
    pub use crate::delete_role::*;
}
mod role_permissions {
    pub use crate::role_permissions::*;
}
mod update_role {
    pub use crate::update_role::*;
}
