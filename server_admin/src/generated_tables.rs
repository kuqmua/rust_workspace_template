#![allow(clippy::needless_for_each, clippy::partial_pub_fields)] // generated contracts expose operation fields while source table fields stay private to protect password hashes

#[path = "generated_tables/admin_generated_route_contract.rs"]
mod admin_generated_route_contract;
#[path = "generated_tables/admin_generated_table.rs"]
mod admin_generated_table;
#[path = "generated_tables/admin_generated_tables_validation_error.rs"]
mod admin_generated_tables_validation_error;
#[path = "generated_tables/admin_permissions.rs"]
mod admin_permissions;
#[path = "generated_tables/admin_role_permissions.rs"]
mod admin_role_permissions;
#[path = "generated_tables/admin_roles.rs"]
mod admin_roles;
#[path = "generated_tables/admin_system_settings.rs"]
mod admin_system_settings;
#[path = "generated_tables/admin_user_roles.rs"]
mod admin_user_roles;
#[path = "generated_tables/admin_users.rs"]
mod admin_users;
#[path = "generated_tables/generated_open_api.rs"]
mod generated_open_api;
#[path = "generated_tables/generated_routes.rs"]
mod generated_routes;
#[path = "generated_tables/shared_admin_generated_table_state_arc.rs"]
mod shared_admin_generated_table_state_arc;
#[path = "generated_tables/utoipa_admin_open_api.rs"]
mod utoipa_admin_open_api;
#[path = "generated_tables/validate_catalog_schema.rs"]
mod validate_catalog_schema;

pub(crate) use admin_generated_route_contract::AdminGeneratedRouteContract;
pub(crate) use admin_generated_table::AdminGeneratedTable;
pub use admin_generated_tables_validation_error::AdminGeneratedTablesValidationError;
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

#[cfg(test)]
#[path = "domain_types_generated_tables_tests.rs"]
mod tests;
