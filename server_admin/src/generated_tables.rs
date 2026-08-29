pub(crate) use admin_generated_route_contract::AdminGeneratedRouteContract;
pub(crate) use admin_generated_table::AdminGeneratedTable;
pub use admin_permissions::*;
pub use admin_role_permissions::*;
pub use admin_roles::*;
pub use admin_system_settings::*;
pub use admin_user_roles::*;
pub use admin_users::*;
pub use generated_open_api::generated_open_api;
pub use generated_routes::generated_routes;
pub use shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc;
pub use utoipa_admin_open_api::*;
pub use validate_catalog_schema::validate_catalog_schema;

// Root-owned module compatibility wrappers.
mod admin_generated_route_contract {
    pub use crate::admin_generated_route_contract::*;
}
mod admin_generated_table {
    pub use crate::admin_generated_table::*;
}
mod admin_permissions {
    pub use crate::admin_permissions::*;
}
mod admin_role_permissions {
    pub use crate::admin_role_permissions::*;
}
mod admin_roles {
    pub use crate::admin_roles::*;
}
mod admin_system_settings {
    pub use crate::admin_system_settings::*;
}
mod admin_user_roles {
    pub use crate::admin_user_roles::*;
}
mod admin_users {
    pub use crate::admin_users::*;
}
mod generated_open_api {
    pub use crate::generated_open_api::*;
}
mod generated_routes {
    pub use crate::generated_routes::*;
}
mod shared_admin_generated_table_state_arc {
    pub use crate::shared_admin_generated_table_state_arc::*;
}
mod utoipa_admin_open_api {
    pub use crate::utoipa_admin_open_api::*;
}
mod validate_catalog_schema {
    pub use crate::validate_catalog_schema::*;
}
