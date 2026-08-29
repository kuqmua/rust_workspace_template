pub use super::admin_permissions::*;
pub use super::admin_role_permissions::*;
pub use super::admin_roles::*;
pub use super::admin_system_settings::*;
pub use super::admin_user_roles::*;
pub use super::admin_users::*;
pub use super::utoipa_admin_open_api::*;
pub(crate) use admin_generated_route_contract::AdminGeneratedRouteContract;
pub(crate) use admin_generated_table::AdminGeneratedTable;
pub use generated_open_api::generated_open_api;
pub use generated_routes::generated_routes;
pub use shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc;
pub use validate_catalog_schema::validate_catalog_schema;

// Root-owned module compatibility wrappers.
mod admin_generated_route_contract {
    pub use super::super::admin_generated_route_contract::*;
}
mod admin_generated_table {
    pub use super::super::admin_generated_table::*;
}
mod admin_permissions {
    pub use super::super::admin_permissions::*;
}
mod admin_role_permissions {
    pub use super::super::admin_role_permissions::*;
}
mod admin_roles {
    pub use super::super::admin_roles::*;
}
mod admin_system_settings {
    pub use super::super::admin_system_settings::*;
}
mod admin_user_roles {
    pub use super::super::admin_user_roles::*;
}
mod admin_users {
    pub use super::super::admin_users::*;
}
mod generated_open_api {
    pub use super::super::generated_open_api::*;
}
mod generated_routes {
    pub use super::super::generated_routes::*;
}
mod shared_admin_generated_table_state_arc {
    pub use super::super::shared_admin_generated_table_state_arc::*;
}
mod utoipa_admin_open_api {
    pub use super::super::utoipa_admin_open_api::*;
}
mod validate_catalog_schema {
    pub use super::super::validate_catalog_schema::*;
}
