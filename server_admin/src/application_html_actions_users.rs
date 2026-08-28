pub(crate) use admin_html_user_action_router::admin_html_user_action_router;

// Root-owned module compatibility wrappers.
mod admin_html_user_action_route_registry {
    pub use crate::admin_html_user_action_route_registry::*;
}
mod admin_html_user_action_router {
    pub use crate::admin_html_user_action_router::*;
}
mod create_user {
    pub use crate::create_user::*;
}
mod delete_user {
    pub use crate::delete_user::*;
}
mod update_user {
    pub use crate::update_user::*;
}
mod user_ban {
    pub use crate::user_ban::*;
}
mod user_password {
    pub use crate::user_password::*;
}
mod user_roles {
    pub use crate::user_roles::*;
}
