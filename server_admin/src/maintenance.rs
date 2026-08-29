use super::{
    AdminDisplayName, AdminLogin, AdminPasswordHashError, AdminPasswordHasher, AdminPermission,
    AdminUserId, SqlxAdminError,
};

pub use super::admin_audit_action::*;
pub use super::admin_audit_resource::*;
pub use super::admin_cleanup_batch_size::*;
pub use super::admin_cleanup_cfg::*;
pub use super::admin_cleanup_cfg_error::*;
pub use super::admin_cleanup_error::*;
pub use super::admin_cleanup_report::*;
pub use super::admin_cleanup_retention_seconds::*;
pub use super::admin_cleanup_rows::*;
pub use super::admin_migrate_error::*;
pub use super::admin_password_reset_error::*;
pub use super::cleanup_admin_tables::*;
pub use super::create_initial_administrator::*;
pub use super::initial_administrator_creation_error::*;
pub use super::prepare_postgresql::*;
pub use super::reset_admin_password::*;
pub use super::sqlx_admin_migrate_error::*;
use admin_migrate_error_inner::AdminMigrateErrorInner;
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
    pub use super::super::admin_audit_action::*;
}
mod admin_audit_resource {
    pub use super::super::admin_audit_resource::*;
}
mod sqlx_admin_migrate_error {
    pub use super::super::sqlx_admin_migrate_error::*;
}
mod admin_migrate_error_inner {
    pub use super::super::admin_migrate_error_inner::*;
}
mod admin_migrate_error {
    pub use super::super::admin_migrate_error::*;
}
mod prepare_postgresql {
    pub use super::super::prepare_postgresql::*;
}
mod admin_cleanup_batch_size {
    pub use super::super::admin_cleanup_batch_size::*;
}
mod admin_cleanup_retention_seconds {
    pub use super::super::admin_cleanup_retention_seconds::*;
}
mod admin_cleanup_cfg {
    pub use super::super::admin_cleanup_cfg::*;
}
mod admin_cleanup_report {
    pub use super::super::admin_cleanup_report::*;
}
mod admin_cleanup_rows {
    pub use super::super::admin_cleanup_rows::*;
}
mod admin_cleanup_cfg_error {
    pub use super::super::admin_cleanup_cfg_error::*;
}
mod admin_cleanup_error {
    pub use super::super::admin_cleanup_error::*;
}
mod cleanup_admin_tables {
    pub use super::super::cleanup_admin_tables::*;
}
mod initial_administrator_creation_error {
    pub use super::super::initial_administrator_creation_error::*;
}
mod admin_password_reset_error {
    pub use super::super::admin_password_reset_error::*;
}
mod create_initial_administrator {
    pub use super::super::create_initial_administrator::*;
}
mod reset_admin_password {
    pub use super::super::reset_admin_password::*;
}
