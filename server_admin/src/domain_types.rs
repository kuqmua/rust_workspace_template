#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
#[path = "application_auth.rs"]
pub mod auth;
#[path = "generated_auth.rs"]
mod generated_auth;
#[path = "generated_tables.rs"]
pub mod generated_tables;
#[path = "hash_opaque_token.rs"]
mod hash_opaque_token;
#[path = "password.rs"]
mod password;
#[path = "rbac.rs"]
mod rbac;
pub use generated_auth::{AdminGeneratedAuthLayer, AdminGeneratedAuthService};
pub use pg_table::domain_types::CombinationOfAppStateLogicTraits;
pub use server_admin_contract::domain_types::{
    AdminDisplayName, AdminLogin, AdminPermission, AdminPermissionTryFromStrError, AdminRoleName,
};
pub use server_admin_core::domain_types::{
    AdminAuditLogId, AdminIdTryFromI64Error, AdminNonZeroUsize, AdminPermissionId,
    AdminPermissionName, AdminRoleId, AdminSocketAddr, AdminUserId, SecrecyAdminString,
    StdAdminBool, StdAdminStrRef, StdAdminString, UuidAdminValue,
};
#[path = "maintenance.rs"]
mod maintenance;
#[path = "security.rs"]
mod security;

pub use maintenance::*;
pub use security::*;
#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "repository policy forbids for loops and compact fixtures keep secret setup deterministic"
)]
#[path = "domain_types_tests.rs"]
mod tests;
