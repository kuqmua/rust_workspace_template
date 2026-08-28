use crate::{
    AdminDisplayName, AdminLogin, AdminPasswordHashError, AdminPasswordHasher, AdminPermission,
    AdminUserId, SqlxAdminError,
};

pub use admin_audit_action::*;
pub use admin_audit_resource::*;
pub use admin_cleanup_batch_size::*;
pub use admin_cleanup_cfg::*;
pub use admin_cleanup_cfg_error::*;
pub use admin_cleanup_error::*;
pub use admin_cleanup_report::*;
pub use admin_cleanup_retention_seconds::*;
pub use admin_cleanup_rows::*;
pub use admin_migrate_error::*;
use admin_migrate_error_inner::AdminMigrateErrorInner;
pub use admin_password_reset_error::*;
pub use cleanup_admin_tables::*;
pub use create_initial_administrator::*;
pub use initial_administrator_creation_error::*;
pub use prepare_postgresql::*;
pub use reset_admin_password::*;
pub use sqlx_admin_migrate_error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn cleanup_batch_rejects_zero() {
        assert_eq!(
            crate::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
            Err(crate::AdminCleanupCfgError::BatchSizeOutOfRange),
        );
    }
}

// Root-owned module compatibility wrappers.
mod admin_audit_action {
    pub use crate::admin_audit_action::*;
}
mod admin_audit_resource {
    pub use crate::admin_audit_resource::*;
}
mod sqlx_admin_migrate_error {
    pub use crate::sqlx_admin_migrate_error::*;
}
mod admin_migrate_error_inner {
    pub use crate::admin_migrate_error_inner::*;
}
mod admin_migrate_error {
    pub use crate::admin_migrate_error::*;
}
mod prepare_postgresql {
    pub use crate::prepare_postgresql::*;
}
mod admin_cleanup_batch_size {
    pub use crate::admin_cleanup_batch_size::*;
}
mod admin_cleanup_retention_seconds {
    pub use crate::admin_cleanup_retention_seconds::*;
}
mod admin_cleanup_cfg {
    pub use crate::admin_cleanup_cfg::*;
}
mod admin_cleanup_report {
    pub use crate::admin_cleanup_report::*;
}
mod admin_cleanup_rows {
    pub use crate::admin_cleanup_rows::*;
}
mod admin_cleanup_cfg_error {
    pub use crate::admin_cleanup_cfg_error::*;
}
mod admin_cleanup_error {
    pub use crate::admin_cleanup_error::*;
}
mod cleanup_admin_tables {
    pub use crate::cleanup_admin_tables::*;
}
mod initial_administrator_creation_error {
    pub use crate::initial_administrator_creation_error::*;
}
mod admin_password_reset_error {
    pub use crate::admin_password_reset_error::*;
}
mod create_initial_administrator {
    pub use crate::create_initial_administrator::*;
}
mod reset_admin_password {
    pub use crate::reset_admin_password::*;
}
