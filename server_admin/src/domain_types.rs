#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
#[path = "application/auth.rs"]
pub mod auth;
mod generated_auth;
pub mod generated_tables;
mod hash_opaque_token;
mod password;
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
mod maintenance;
mod security;

pub use maintenance::*;
pub use security::*;
#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "repository policy forbids for loops and compact fixtures keep secret setup deterministic"
)]
mod tests;
