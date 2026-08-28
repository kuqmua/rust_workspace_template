// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
pub use generated_auth::admin_generated_auth_layer::AdminGeneratedAuthLayer;
pub use generated_auth::admin_generated_auth_service::AdminGeneratedAuthService;
pub use pg_table::domain_types::CombinationOfAppStateLogicTraits;
pub use server_admin_contract::domain_types::{
    AdminDisplayName, AdminLogin, AdminPermission, AdminPermissionTryFromStrError, AdminRoleName,
};
pub use server_admin_core::domain_types::{
    AdminAuditLogId, AdminIdTryFromI64Error, AdminNonZeroUsize, AdminPermissionId,
    AdminPermissionName, AdminRoleId, AdminSocketAddr, AdminUserId, SecrecyAdminString,
    StdAdminBool, StdAdminStrRef, StdAdminString, UuidAdminValue,
};

pub use maintenance::*;
pub use security::*;

// Root-owned module compatibility wrappers.
pub mod auth {
    pub use crate::application_auth::*;
}
mod generated_auth {
    pub use crate::generated_auth::*;
}
pub mod generated_tables {
    pub use crate::generated_tables::*;
}
mod hash_opaque_token {
    pub use crate::hash_opaque_token::*;
}
mod password {
    pub use crate::password::*;
}
mod rbac {
    pub use crate::rbac::*;
}
mod maintenance {
    pub use crate::maintenance::*;
}
mod security {
    pub use crate::security::*;
}
