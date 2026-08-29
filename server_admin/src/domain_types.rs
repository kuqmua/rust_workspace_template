// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
pub use generated_auth::admin_generated_auth_layer::AdminGeneratedAuthLayer;
pub use generated_auth::admin_generated_auth_service::AdminGeneratedAuthService;
pub use pg_table::CombinationOfAppStateLogicTraits;
pub use server_admin_contract::domain_types::{
    AdminDisplayName, AdminLogin, AdminPermission, AdminPermissionTryFromStrError, AdminRoleName,
};
pub use server_admin_core::domain_types::{
    AdminAuditLogId, AdminIdTryFromI64Error, AdminPermissionId, AdminRoleId, AdminSocketAddr,
    AdminUserId, SecrecyAdminString, StdAdminBool, StdAdminStrRef, StdAdminString, UuidAdminValue,
};

pub use super::maintenance::*;
pub use super::security::*;
// Root-owned module compatibility wrappers.
pub mod auth {
    pub use super::super::application_auth::*;
}
mod generated_auth {
    pub use super::super::generated_auth::*;
}
pub mod generated_tables {
    pub use super::super::generated_tables::*;
}
mod hash_opaque_token {
    pub use super::super::hash_opaque_token::*;
}
mod rbac {
    pub use super::super::rbac::*;
}
mod maintenance {
    pub use super::super::maintenance::*;
}
mod security {
    pub use super::super::security::*;
}
